# Storage Macros

Procedural macros helper for interacting with `cw-storage-plus` and `cosmwasm-storage`.

## Current features

### Index List Impl Macros

Auto generate `IndexList` impl for your indexes struct.

`index_list_impl(T)` will generate `impl IndexList<T>` for struct below the macro's call.

`IndexList`, `Index` imports from `cw-storage-plus` are also required.

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct TestStruct {
    id: u64,
    id2: u32,
    addr: Addr,
}

#[index_list_impl(TestStruct)] // <- Add this line right here, 
struct TestIndexes<'a> {
    id: MultiIndex<'a, (U32Key, Vec<u8>), TestStruct>,
    addr: UniqueIndex<'a, Addr, TestStruct>,
}
```

