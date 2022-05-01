mod indexed_map;
mod item;
mod map;

#[cfg(test)]
mod tests;

pub use indexed_map::{IndexedMapCow, MultiIndexCow, UniqueIndexCow};
pub use item::ItemCow;
pub use map::MapCow;

pub trait StorageCow<'key> {
    fn new_owned(namespace: String) -> Self;

    fn new_ref(namespace: &'key str) -> Self;
}
