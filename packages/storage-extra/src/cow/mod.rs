mod item;
mod map;

pub use item::ItemCow;
pub use map::MapCow;

pub trait StorageCow<'key> {
    fn new_owned(namespace: String) -> Self;

    fn new_ref(namespace: &'key str) -> Self;
}

#[cfg(test)]
mod combined_test {
    use cosmwasm_std::Addr;

    use super::*;

    struct ItemMapAccessor<'a, 'key> {
        item: ItemCow<'a, u64>,
        map: MapCow<'a, &'key Addr, u64>,
    }

    impl ItemMapAccessor<'_, '_> {
        fn new(primary_ns: &str) -> Self {
            Self {
                item: ItemCow::new_owned(format!("{}-item", primary_ns)),
                map: MapCow::new_owned(format!("{}-map", primary_ns)),
            }
        }
    }

    #[test]
    fn init() {
        let it = ItemMapAccessor::new("primary");
        assert_eq!(it.item.namespace, "primary-item");
        assert_eq!(it.map.namespace, "primary-map");
    }
}
