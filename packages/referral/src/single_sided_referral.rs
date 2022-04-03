use cosmwasm_std::{Addr, Order, StdError, StdResult, Storage};
use cw_storage_plus::{Bound, Map};

use crate::referral::{Refer, DEFAULT_ALL_LIMIT, DEFAULT_DEPTH};

pub struct SingleSidedReferral<'a>(Map<'a, &'a Addr, Addr>);

impl<'a> SingleSidedReferral<'a> {
    pub const fn new(map_namespace: &'a str) -> Self {
        SingleSidedReferral(Map::new(map_namespace))
    }

    pub fn set_ref(
        &self,
        storage: &mut dyn Storage,
        referred_addr: &Addr,
        referrer_addr: &Addr,
    ) -> StdResult<()> {
        if referred_addr == referrer_addr {
            return Err(StdError::generic_err(
                "Referrer can not be same address as referral",
            ));
        }

        self.0.update(storage, referred_addr, |r| -> StdResult<_> {
            match r {
                Some(_) => Err(StdError::generic_err("This address already has referral")),
                None => Ok(referrer_addr.clone()),
            }
        })?;

        Ok(())
    }

    pub fn ref_chains(
        &self,
        storage: &dyn Storage,
        addr: &Addr,
        depth: Option<u64>,
    ) -> StdResult<Vec<Addr>> {
        let mut chains: Vec<Addr> = vec![];

        for _ in 0..depth.unwrap_or(DEFAULT_DEPTH) {
            if let Some(r_addr) = self.0.may_load(storage, chains.last().unwrap_or(addr))? {
                chains.push(r_addr);
            } else {
                break;
            }
        }

        Ok(chains)
    }

    pub fn ref_of(&self, storage: &dyn Storage, addr: &Addr) -> StdResult<Option<Addr>> {
        self.0.may_load(storage, addr)
    }

    pub fn has_ref(&self, storage: &dyn Storage, addr: &Addr) -> StdResult<bool> {
        Ok(self.ref_of(storage, addr)?.is_some())
    }

    pub fn all_ref(
        &self,
        storage: &dyn Storage,
        start_after: Option<Addr>,
        limit: Option<u64>,
        is_ascending: Option<bool>,
    ) -> Vec<Refer> {
        let bound = match is_ascending.unwrap_or(true) {
            true => (
                start_after
                    .as_ref()
                    .map(|e| Bound::Exclusive(e.as_bytes().to_vec())),
                None,
                Order::Ascending,
            ),
            false => (
                None,
                start_after
                    .as_ref()
                    .map(|e| Bound::Exclusive(e.as_bytes().to_vec())),
                Order::Descending,
            ),
        };

        self.0
            .range(storage, bound.0, bound.1, bound.2)
            .map(|e| {
                let buf = e.unwrap();
                Refer {
                    referrer: buf.1,
                    referred: Addr::unchecked(String::from_utf8(buf.0).unwrap()),
                }
            })
            .take(limit.unwrap_or(DEFAULT_ALL_LIMIT) as usize)
            .collect::<Vec<_>>()
    }
}
