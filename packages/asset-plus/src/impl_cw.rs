use cosmwasm_std::{
    from_slice, to_binary, to_vec, Api, BankMsg, Coin, CosmosMsg, QuerierWrapper, StdError,
    StdResult, Uint128, WasmMsg,
};
use cw20::{BalanceResponse, Cw20ExecuteMsg, Cw20QueryMsg};
use cw_storage_plus::PrimaryKey;

use crate::{asset::AssetInfo, Asset};

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

impl Asset {
    pub fn new<A: Into<Uint128>>(info: AssetInfo, amount: A) -> Self {
        Asset {
            info,
            amount: amount.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.amount.is_zero()
    }

    pub fn transfer_all_msg<T: Into<String>>(&self, to_address: T) -> StdResult<CosmosMsg> {
        self.info.transfer_msg(to_address, self.amount)
    }

    pub fn assert_sent_token(&self, coins: &[Coin]) -> StdResult<()> {
        if let AssetInfo::NativeToken { denom } = &self.info {
            match coins.iter().find(|c| &c.denom == denom) {
                Some(c) => (c.amount == self.amount).then(|| ()).ok_or_else(|| {
                    StdError::generic_err(format!(
                        "Expected amount {} but found {} for denom {}",
                        self.amount, c.amount, denom
                    ))
                }),
                None => Err(StdError::generic_err(format!(
                    "Denom {} not found in sent coins",
                    denom
                ))),
            }?;
        }

        Ok(())
    }
}

impl AssetInfo {
    pub fn as_bytes(&self) -> &[u8] {
        match &self {
            AssetInfo::Token { contract_addr } => contract_addr.as_bytes(),
            AssetInfo::NativeToken { denom } => denom.as_bytes(),
        }
    }

    pub fn from_bytes(b: &[u8], api: &dyn Api) -> StdResult<Self> {
        let s = String::from_utf8(b.to_vec())
            .map_err(|_| StdError::invalid_utf8("String parsing error"))?;
        Ok(match api.addr_validate(&s) {
            Ok(addr) => AssetInfo::Token {
                contract_addr: addr,
            },
            Err(_) => AssetInfo::NativeToken { denom: s },
        })
    }

    pub fn to_serde_vec(&self) -> StdResult<Vec<u8>> {
        to_vec(self)
    }

    pub fn from_serde_slice(b: &[u8]) -> StdResult<Self> {
        from_slice(b)
    }

    pub fn transfer_msg<T: Into<String>, A: Into<Uint128>>(
        &self,
        to_address: T,
        amount: A,
    ) -> StdResult<CosmosMsg> {
        let msg = match self {
            AssetInfo::Token { contract_addr } => CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: contract_addr.into(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: to_address.into(),
                    amount: amount.into(),
                })?,
                funds: vec![],
            }),
            AssetInfo::NativeToken { denom } => CosmosMsg::Bank(BankMsg::Send {
                to_address: to_address.into(),
                amount: vec![Coin {
                    denom: denom.clone(),
                    amount: amount.into(),
                }],
            }),
        };

        Ok(msg)
    }

    pub fn query_balance<T: Into<String>>(
        &self,
        querier: &QuerierWrapper,
        address: T,
    ) -> StdResult<Uint128> {
        match self {
            AssetInfo::Token { contract_addr } => {
                let bal: BalanceResponse = querier.query_wasm_smart(
                    contract_addr,
                    &Cw20QueryMsg::Balance {
                        address: address.into(),
                    },
                )?;
                Ok(bal.balance)
            }
            AssetInfo::NativeToken { denom } => Ok(querier.query_balance(address, denom)?.amount),
        }
    }
}
