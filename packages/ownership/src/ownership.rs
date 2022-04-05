use crate::error::ContractError;
use crate::msg::{
  OwnerResponse, NextOwnerResponse
};
use crate::state::{
  OWNERSHIP, Ownership
};
use cosmwasm_std::{
    Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Order, Response, StdResult, WasmMsg, Empty
};

pub fn only_owner(deps: &DepsMut, info: MessageInfo) -> StdResult<bool> {
  let ownership = OWNERSHIP.load(deps.storage)?;
  let sender = deps.api.addr_canonicalize(info.sender.as_str())?;
  if sender == ownership.owner {
    return Ok(true);
  }
  return Err(StdError::NotOwner());
}

pub fn only_next_owner(deps: &DepsMut, _env: Env, info: MessageInfo) -> StdResult<bool> {
  let ownership = OWNERSHIP.load(deps.storage)?;
  let sender = deps.api.addr_canonicalize(info.sender.as_str())?;
  if sender == ownership.next_owner {
    return Ok(true);
  }
  return Err(StdError::NotOwner());
}

pub fn set_next_owner(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  address: Addr,
) -> StdResult<Response> {
  only_owner(&deps, info)?;
  let next_owner = deps.api.addr_canonicalize(&address.as_str())?;
  let ownership = OWNERSHIP.load(deps.storage)?;
  ownership.next_owner = next_owner;
  ownership.save(deps.storage, &ownership);

  Ok(Response::default())
}

pub fn accept_owner(deps: DepsMut, _env: Env, info: MessageInfo) -> StdResult<Response> {
  only_next_owner(&deps, _env, info)?;
  let ownership = OWNERSHIP.load(deps.storage)?;
  ownership.owner = CanonicalAddr::from(cfg.next_owner.as_slice());
  ownership.save(deps.storage, &ownership);

  Ok(Response::default())
}