use cosmwasm_std::{testing::MockStorage, Addr};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex, U32Key, U64Key, UniqueIndex};
use serde::{Deserialize, Serialize};
use tw_storage_macros::index_list_impl;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct TestStruct {
    id: u64,
    id2: u32,
    addr: Addr,
}

#[index_list_impl(TestStruct)]
struct TestIndexes<'a> {
    id: MultiIndex<'a, (U32Key, Vec<u8>), TestStruct>,
    addr: UniqueIndex<'a, Addr, TestStruct>,
}

#[test]
fn compile() {
    let _: IndexedMap<U64Key, TestStruct, TestIndexes> = IndexedMap::new(
        "t",
        TestIndexes {
            id: MultiIndex::new(|t, k| (t.id2.into(), k), "t", "t_2"),
            addr: UniqueIndex::new(|t| t.addr.clone(), "t_addr"),
        },
    );
}

#[test]
fn works() {
    let mut storage = MockStorage::new();
    let idm: IndexedMap<U64Key, TestStruct, TestIndexes> = IndexedMap::new(
        "t",
        TestIndexes {
            id: MultiIndex::new(|t, k| (t.id2.into(), k), "t", "t_2"),
            addr: UniqueIndex::new(|t| t.addr.clone(), "t_addr"),
        },
    );

    idm.save(
        &mut storage,
        0.into(),
        &TestStruct {
            id: 0,
            id2: 100,
            addr: Addr::unchecked("1"),
        },
    )
    .unwrap();

    assert_eq!(
        idm.load(&storage, 0.into()).unwrap(),
        TestStruct {
            id: 0,
            id2: 100,
            addr: Addr::unchecked("1"),
        }
    );
}
