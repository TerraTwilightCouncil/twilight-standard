use cosmwasm_std::entry_point;
use cosmwasm_std::{
  Api, CanonicalAddr, Env, Deps ,DepsMut, Response, HumanAddr, Querier, StdError, StdResult, Storage,
  MessageInfo, Addr
};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ownership {
  pub owner: CanonicalAddr,
  pub next_owner: CanonicalAddr,
}

pub const OWNERSHIP: Item<Ownership> = Item::new("ownership");