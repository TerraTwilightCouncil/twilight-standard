use cw_storage_plus::PrimaryKey;

use crate::asset::AssetInfo;

impl PrimaryKey<'_> for AssetInfo {
    type Prefix = ();

    type SubPrefix = ();

    fn key(&self) -> Vec<&[u8]> {
        vec![self.as_bytes()]
    }
}

impl<'a> PrimaryKey<'a> for &'a AssetInfo {
    type Prefix = ();

    type SubPrefix = ();

    fn key(&self) -> Vec<&[u8]> {
        vec![self.as_bytes()]
    }
}
