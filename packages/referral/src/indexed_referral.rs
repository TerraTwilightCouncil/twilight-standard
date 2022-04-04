use cosmwasm_std::{Addr, Order, StdError, StdResult, Storage};
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, MultiIndex};

use crate::referral::{Refer, DEFAULT_ALL_LIMIT, DEFAULT_DEPTH, DEFAULT_REFERRED_LIMIT};

pub struct ReferralIndexes<'a> {
    pub referred: MultiIndex<'a, (Addr, Vec<u8>), Refer>,
}

impl IndexList<Refer> for ReferralIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Refer>> + '_> {
        let v: Vec<&dyn Index<Refer>> = vec![&self.referred];
        Box::new(v.into_iter())
    }
}

pub struct IndexedReferral<'a>(IndexedMap<'a, &'a Addr, Refer, ReferralIndexes<'a>>);

impl<'a> IndexedReferral<'a> {
    pub fn new(ref_namespace: &'a str, ref_index_namespace: &'a str) -> Self {
        IndexedReferral(IndexedMap::new(
            ref_namespace,
            ReferralIndexes {
                referred: MultiIndex::new(
                    |refer, key| (refer.referrer.clone(), key),
                    ref_namespace,
                    ref_index_namespace,
                ),
            },
        ))
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
                None => Ok(Refer {
                    referrer: referrer_addr.clone(),
                    referred: referred_addr.clone(),
                }),
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
            match self.0.may_load(storage, chains.last().unwrap_or(addr))? {
                Some(r_addr) => chains.push(r_addr.referrer),
                None => break,
            };
        }

        Ok(chains)
    }

    pub fn ref_of(&self, storage: &dyn Storage, addr: &Addr) -> StdResult<Option<Addr>> {
        Ok(self.0.may_load(storage, addr)?.map(|r| r.referrer))
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
            .map(|e| e.unwrap().1)
            .take(limit.unwrap_or(DEFAULT_ALL_LIMIT) as usize)
            .collect::<Vec<_>>()
    }

    pub fn all_referred_of(
        &self,
        storage: &dyn Storage,
        addr: Addr,
        start_after: Option<Addr>,
        limit: Option<u64>,
        is_ascending: Option<bool>,
    ) -> StdResult<Vec<Addr>> {
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

        Ok(self
            .0
            .idx
            .referred
            .prefix(addr)
            .keys(storage, bound.0, bound.1, bound.2)
            .map(|e| Addr::unchecked(String::from_utf8(e).unwrap()))
            .take(limit.unwrap_or(DEFAULT_REFERRED_LIMIT) as usize)
            .collect::<Vec<_>>())
    }
}
