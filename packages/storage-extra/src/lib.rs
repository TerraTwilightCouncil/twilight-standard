use std::{borrow::Cow, marker::PhantomData};

use cw_storage_plus::{Map, PrimaryKey};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct MapCow<'a, K, T> {
    namespace: Cow<'a, str>,
    key_type: PhantomData<K>,
    data_type: PhantomData<T>,
}

impl<'a, 'key, K, T> MapCow<'a, K, T>
where
    T: Serialize + DeserializeOwned,
    K: PrimaryKey<'key>,
    'key: 'a,
{
    pub const fn new_owned(namespace: String) -> Self {
        Self {
            namespace: Cow::Owned(namespace),
            key_type: PhantomData,
            data_type: PhantomData,
        }
    }

    pub const fn new_ref(namespace: &'key str) -> Self {
        Self {
            namespace: Cow::Borrowed(namespace),
            key_type: PhantomData,
            data_type: PhantomData,
        }
    }

    pub fn map(&'a self) -> Map<'a, K, T> {
        Map::new(&self.namespace)
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{testing::MockStorage, Addr};

    use crate::MapCow;

    #[test]
    fn new_owned() {
        let a = Addr::unchecked("g");
        let mut storage = MockStorage::new();
        let addr_owned: MapCow<&Addr, u64> = MapCow::new_owned(String::from("g"));

        addr_owned.map().save(&mut storage, &a, &1).unwrap();
        assert_eq!(addr_owned.map().load(&storage, &a).unwrap(), 1);
    }

    #[test]
    fn new_ref() {
        let a = Addr::unchecked("g");
        let mut storage = MockStorage::new();
        const ADDR_REF: MapCow<&Addr, u64> = MapCow::new_ref("g");

        ADDR_REF.map().save(&mut storage, &a, &1).unwrap();
        assert_eq!(ADDR_REF.map().load(&storage, &a).unwrap(), 1);
    }
}
