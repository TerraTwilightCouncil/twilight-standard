use cosmwasm_std::{from_slice, Pair, StdError, StdResult, Storage};
use cw_storage_plus::{Index, Map, Prefix, Prefixer, PrimaryKey};
use serde::{de::DeserializeOwned, Serialize};

use super::helpers::namespaces_with_key;

type DeserializeFn<T> = fn(&dyn Storage, &[u8], Pair) -> StdResult<Pair<T>>;

#[derive(Clone)]
pub struct CustomDeseMultiIndex<'a, K, T> {
    idx_fn: fn(&T, Vec<u8>) -> K,
    dese_fn: Option<DeserializeFn<T>>,
    idx_namespace: &'a [u8],
    idx_map: Map<'a, K, u32>,
    pk_namespace: &'a [u8],
}

impl<'a, K, T> CustomDeseMultiIndex<'a, K, T> {
    pub const fn new(
        idx_fn: fn(&T, Vec<u8>) -> K,
        dese_fn: Option<DeserializeFn<T>>,
        pk_namespace: &'a str,
        idx_namespace: &'a str,
    ) -> Self {
        Self {
            idx_fn,
            dese_fn,
            idx_namespace: idx_namespace.as_bytes(),
            idx_map: Map::new(idx_namespace),
            pk_namespace: pk_namespace.as_bytes(),
        }
    }
}

fn deserialize_multi_kv<T: DeserializeOwned>(
    store: &dyn Storage,
    pk_namespace: &[u8],
    kv: Pair,
) -> StdResult<Pair<T>> {
    let (key, pk_len) = kv;

    // Deserialize pk_len
    let pk_len = from_slice::<u32>(pk_len.as_slice())?;

    // Recover pk from last part of k
    let offset = key.len() - pk_len as usize;
    let pk = &key[offset..];

    let full_key = namespaces_with_key(&[pk_namespace], pk);

    let v = store
        .get(&full_key)
        .ok_or_else(|| StdError::generic_err("pk not found"))?;
    let v = from_slice::<T>(&v)?;

    Ok((pk.into(), v))
}

pub fn deserialize_multi_kv_custom_pk<T: DeserializeOwned>(
    store: &dyn Storage,
    pk_namespace: &[u8],
    kv: Pair,
    pk_fn: fn(Vec<u8>) -> Vec<u8>,
) -> StdResult<Pair<T>> {
    let (key, pk_len) = kv;

    // Deserialize pk_len
    let pk_len = from_slice::<u32>(pk_len.as_slice())?;

    // Recover pk from last part of k
    let offset = key.len() - pk_len as usize;
    let pk = pk_fn(key[offset..].to_vec());

    let full_key = namespaces_with_key(&[pk_namespace], pk.as_slice());

    let v = store
        .get(&full_key)
        .ok_or_else(|| StdError::generic_err("pk not found"))?;
    let v = from_slice::<T>(&v)?;

    Ok((pk, v))
}

impl<'a, K, T> Index<T> for CustomDeseMultiIndex<'a, K, T>
where
    T: Serialize + DeserializeOwned + Clone,
    K: PrimaryKey<'a>,
{
    fn save(&self, store: &mut dyn Storage, pk: &[u8], data: &T) -> StdResult<()> {
        let idx = (self.idx_fn)(data, pk.to_vec());
        self.idx_map.save(store, idx, &(pk.len() as u32))
    }

    fn remove(&self, store: &mut dyn Storage, pk: &[u8], old_data: &T) -> StdResult<()> {
        let idx = (self.idx_fn)(old_data, pk.to_vec());
        self.idx_map.remove(store, idx);
        Ok(())
    }
}

impl<'a, K, T> CustomDeseMultiIndex<'a, K, T>
where
    T: Serialize + DeserializeOwned + Clone,
    K: PrimaryKey<'a>,
{
    pub fn prefix(&self, p: K::Prefix) -> Prefix<T> {
        Prefix::with_deserialization_function(
            self.idx_namespace,
            &p.prefix(),
            self.pk_namespace,
            match self.dese_fn {
                Some(f) => f,
                None => deserialize_multi_kv,
            },
        )
    }

    pub fn sub_prefix(&self, p: K::SubPrefix) -> Prefix<T> {
        Prefix::with_deserialization_function(
            self.idx_namespace,
            &p.prefix(),
            self.pk_namespace,
            match self.dese_fn {
                Some(f) => f,
                None => deserialize_multi_kv,
            },
        )
    }

    pub fn index_key(&self, k: K) -> Vec<u8> {
        k.joined_key()
    }
}

#[cfg(test)]
mod custom_dese_test {
    use cosmwasm_std::{testing::MockStorage, Order, Uint128};
    use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex, PrimaryKey, U128Key, U64Key};
    use serde::{Deserialize, Serialize};

    use super::{deserialize_multi_kv_custom_pk, CustomDeseMultiIndex};

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
    struct Test {
        id: u64,
        val: Uint128,
    }

    struct TestIndexes<'a> {
        val: CustomDeseMultiIndex<'a, (U128Key, Vec<u8>), Test>,
        val_n: MultiIndex<'a, (U128Key, Vec<u8>), Test>,
    }

    impl IndexList<Test> for TestIndexes<'_> {
        fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Test>> + '_> {
            let v: Vec<&dyn Index<Test>> = vec![&self.val, &self.val_n];
            Box::new(v.into_iter())
        }
    }

    fn idm<'a>() -> IndexedMap<'a, U64Key, Test, TestIndexes<'a>> {
        IndexedMap::new(
            "test",
            TestIndexes {
                val: CustomDeseMultiIndex::new(
                    |t, _| {
                        (
                            t.val.u128().into(),
                            U64Key::new(u64::max_value() - t.id).joined_key(),
                        )
                    },
                    Some(|s, pk, kv| {
                        deserialize_multi_kv_custom_pk(s, pk, kv, |old_kv| {
                            U64Key::new(
                                u64::max_value()
                                    - u64::from_be_bytes(old_kv.as_slice().try_into().unwrap()),
                            )
                            .joined_key()
                        })
                    }),
                    "test",
                    "test__val",
                ),
                val_n: MultiIndex::new(|t, k| (t.val.u128().into(), k), "test", "test__val_n"),
            },
        )
    }

    #[test]
    fn correctly_dese() {
        let mut storage = MockStorage::new();
        idm()
            .save(
                &mut storage,
                0.into(),
                &Test {
                    id: 0,
                    val: Uint128::from(100u64),
                },
            )
            .unwrap();

        let v = idm()
            .idx
            .val
            .sub_prefix(())
            .range(&storage, None, None, Order::Ascending)
            .map(|e| e.unwrap().1.id)
            .collect::<Vec<_>>();

        assert_eq!(v, vec![0]);
    }

    #[test]
    fn index_correctly_use_dese_fn() {
        let mut storage = MockStorage::new();
        idm()
            .save(
                &mut storage,
                0.into(),
                &Test {
                    id: 0,
                    val: Uint128::from(100u64),
                },
            )
            .unwrap();

        idm()
            .save(
                &mut storage,
                1.into(),
                &Test {
                    id: 1,
                    val: Uint128::from(100u64),
                },
            )
            .unwrap();

        idm()
            .save(
                &mut storage,
                2.into(),
                &Test {
                    id: 2,
                    val: Uint128::from(200u64),
                },
            )
            .unwrap();

        idm()
            .save(
                &mut storage,
                3.into(),
                &Test {
                    id: 3,
                    val: Uint128::from(100u64),
                },
            )
            .unwrap();

        let v = idm()
            .idx
            .val
            .sub_prefix(())
            .range(&storage, None, None, Order::Descending)
            .map(|e| e.unwrap().1.id)
            .collect::<Vec<_>>();

        assert_eq!(v, vec![2, 0, 1, 3]);

        let vn = idm()
            .idx
            .val_n
            .sub_prefix(())
            .range(&storage, None, None, Order::Descending)
            .map(|e| e.unwrap().1.id)
            .collect::<Vec<_>>();

        assert_eq!(vn, vec![2, 3, 1, 0]);
    }
}
