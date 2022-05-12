mod conditional_multi_index;
mod custom_dese_index;
mod helpers;
mod indexed_map;
mod indexed_map_ref;
mod item;
mod map;

#[cfg(test)]
mod tests;

pub use conditional_multi_index::ConditionalMultiIndex;
pub use custom_dese_index::{deserialize_multi_kv_custom_pk, CustomDeseMultiIndex};
pub use indexed_map::{IndexedMapCow, MultiIndexCow, UniqueIndexCow};
pub use indexed_map_ref::IndexedMapRef;
pub use item::ItemCow;
pub use map::MapCow;

pub trait StorageCow<'key> {
    fn new_owned(namespace: String) -> Self;

    fn new_ref(namespace: &'key str) -> Self;
}
