mod indexed_map;
mod item;
mod map;

pub use indexed_map::{IndexedMapCow, MultiIndexCow, UniqueIndexCow};
pub use item::ItemCow;
pub use map::MapCow;

pub trait StorageCow<'key> {
    fn new_owned(namespace: String) -> Self;

    fn new_ref(namespace: &'key str) -> Self;
}

#[cfg(test)]
mod combined_test {
    use cosmwasm_std::Addr;
    use cw_storage_plus::{Index, IndexList, U64Key};
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize, Clone)]
    struct ToIndex {
        id: u64,
        count: u64,
        address: Addr,
    }

    #[derive(Clone)]
    struct ToIndexList<'a> {
        count: MultiIndexCow<'a, (U64Key, Vec<u8>), ToIndex>,
        address: UniqueIndexCow<'a, Addr, ToIndex>,
    }

    impl IndexList<ToIndex> for ToIndexList<'_> {
        fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<ToIndex>> + '_> {
            let v: Vec<&dyn Index<ToIndex>> = vec![&self.count, &self.address];
            Box::new(v.into_iter())
        }
    }

    struct ItemMapAccessor<'a, 'key> {
        item: ItemCow<'a, u64>,
        map: MapCow<'a, &'key Addr, u64>,
        indexed_map: IndexedMapCow<'a, U64Key, ToIndex, ToIndexList<'a>>,
    }

    impl ItemMapAccessor<'_, '_> {
        fn new(primary_ns: &str) -> Self {
            Self {
                item: ItemCow::new_owned(format!("{}-item", primary_ns)),
                map: MapCow::new_owned(format!("{}-map", primary_ns)),
                indexed_map: IndexedMapCow::new_owned(
                    format!("{}-idm", primary_ns),
                    ToIndexList {
                        count: MultiIndexCow::new_owned(
                            format!("{}-idm", primary_ns),
                            format!("{}-idm-count", primary_ns),
                            |e, k| (e.count.into(), k),
                        ),
                        address: UniqueIndexCow::new_owned(
                            format!("{}-idm-addr", primary_ns),
                            |e| e.address.clone(),
                        ),
                    },
                ),
            }
        }
    }

    #[test]
    fn init() {
        let it = ItemMapAccessor::new("primary");
        assert_eq!(it.item.namespace, "primary-item");
        assert_eq!(it.map.namespace, "primary-map");
        assert_eq!(it.indexed_map.pk_namespace, "primary-idm");
        assert_eq!(it.indexed_map.index.count.pk_namespace, "primary-idm");
        assert_eq!(
            it.indexed_map.index.count.idx_namespace,
            "primary-idm-count"
        );
        assert_eq!(
            it.indexed_map.index.address.idx_namespace,
            "primary-idm-addr"
        );
    }
}
