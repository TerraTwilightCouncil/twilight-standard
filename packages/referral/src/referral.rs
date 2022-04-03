use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const DEFAULT_DEPTH: u64 = 3;
pub const DEFAULT_REFERRED_LIMIT: u64 = 50;
pub const DEFAULT_ALL_LIMIT: u64 = 100;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Refer {
    pub referrer: Addr,
    pub referred: Addr,
}
