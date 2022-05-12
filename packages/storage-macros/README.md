# Storage Macros

Procedural macros helper for interacting with `cw-storage-plus` and `cosmwasm-storage`.

## Current features

### Index List Impl Macros

Auto generate `IndexList` impl for your indexes struct.

`index_list_impl(T)` will generate `impl IndexList<T>` for struct `T` below the macro's call.

`IndexList`, `Index` imports from `cw-storage-plus` are also required.

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct TestStruct {
    id: u64,
    id2: u32,
    addr: Addr,
}

#[index_list_impl(TestStruct)] // <- Add this line right here,.
struct TestIndexes<'a> {
    id: MultiIndex<'a, (U32Key, Vec<u8>), TestStruct>,
    addr: UniqueIndex<'a, Addr, TestStruct>,
}
```

### StorageKey, Primary Key And Prefixer Impl Deive Macros

Auto generate `PrimaryKey` and `Prefixer` impl for owned and reference variants, `as_bytes` and `from_slice` impl.

`StorageKey` will generate

- `impl PrimaryKey<'_> for T`
- `impl<'a> PrimaryKey<'a> for &'a T`
- `impl Prefixer<'_> for T`
- `impl<'a> Prefixer<'a> for &'a T`
- `pub fn as_bytes(&self) -> &[u8]`
- `pub fn from_slice(b: &[u8]) -> T`

for enum `T` below the macro's call.

`PrimaryKey`, `Prefixer` imports from `cw-storage-plus` are also required.

```rust
#[derive(Clone, Copy, StorageKey)] // <- Add the derive macro here.
enum TestEnum {
    G,
    F,
}
```r
