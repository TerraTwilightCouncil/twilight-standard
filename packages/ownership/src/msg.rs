use cosmwasm_std::entry_point;
use cosmwasm_std::{
  Api, CanonicalAddr, Env, Addr
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerResponse {
  pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NextOwnerResponse {
  pub next_owner: Addr,
}