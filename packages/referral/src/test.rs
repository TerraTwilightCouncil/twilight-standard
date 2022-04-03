#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_dependencies, Addr};

    use crate::{IndexedReferral, SingleSidedReferral};

    #[test]
    fn test_single_sided_referral() {
        let mut deps = mock_dependencies(&[]);
        let referral: SingleSidedReferral = SingleSidedReferral::new("ref_pk");

        let a = Addr::unchecked("a");
        let bb = Addr::unchecked("bb");
        let b = Addr::unchecked("b");
        let c = Addr::unchecked("c");
        let d = Addr::unchecked("d");

        referral.set_ref(&mut deps.storage, &b, &a).unwrap();
        referral.set_ref(&mut deps.storage, &bb, &a).unwrap();
        referral.set_ref(&mut deps.storage, &a, &a).unwrap_err();

        let chains = referral.ref_chains(&deps.storage, &b, Some(5)).unwrap();

        assert_eq!(chains, vec![a.clone()]);

        referral.set_ref(&mut deps.storage, &c, &b).unwrap();
        referral.set_ref(&mut deps.storage, &d, &c).unwrap();

        let chains = referral.ref_chains(&deps.storage, &d, Some(5)).unwrap();

        assert_eq!(chains, vec![c.clone(), b.clone(), a.clone()]);

        let e = Addr::unchecked("e");
        let f = Addr::unchecked("f");
        let g = Addr::unchecked("g");

        referral.set_ref(&mut deps.storage, &e, &d).unwrap();
        referral.set_ref(&mut deps.storage, &f, &e).unwrap();
        referral.set_ref(&mut deps.storage, &g, &f).unwrap();

        let chains = referral.ref_chains(&deps.storage, &g, Some(5)).unwrap();

        assert_eq!(chains, vec![f, e, d, c, b]);
        assert_eq!(chains.len(), 5);
    }

    #[test]
    fn test_indexed_referral() {
        let mut deps = mock_dependencies(&[]);
        let referral: IndexedReferral = IndexedReferral::new("ref_pk", "ref_idx");

        let a = Addr::unchecked("a");
        let bb = Addr::unchecked("bb");
        let b = Addr::unchecked("b");
        let c = Addr::unchecked("c");
        let d = Addr::unchecked("d");

        referral.set_ref(&mut deps.storage, &b, &a).unwrap();
        referral.set_ref(&mut deps.storage, &bb, &a).unwrap();
        referral.set_ref(&mut deps.storage, &a, &a).unwrap_err();

        let chains = referral.ref_chains(&deps.storage, &b, Some(5)).unwrap();

        assert_eq!(chains, vec![a.clone()]);

        referral.set_ref(&mut deps.storage, &c, &b).unwrap();
        referral.set_ref(&mut deps.storage, &d, &c).unwrap();

        let chains = referral.ref_chains(&deps.storage, &d, Some(5)).unwrap();

        assert_eq!(chains, vec![c.clone(), b.clone(), a.clone()]);

        let e = Addr::unchecked("e");
        let f = Addr::unchecked("f");
        let g = Addr::unchecked("g");

        referral.set_ref(&mut deps.storage, &e, &d).unwrap();
        referral.set_ref(&mut deps.storage, &f, &e).unwrap();
        referral.set_ref(&mut deps.storage, &g, &f).unwrap();

        let chains = referral.ref_chains(&deps.storage, &g, Some(5)).unwrap();

        assert_eq!(chains, vec![f, e, d, c, b.clone()]);
        assert_eq!(chains.len(), 5);

        let h = Addr::unchecked("h");
        let j = Addr::unchecked("j");
        referral.set_ref(&mut deps.storage, &h, &a).unwrap();
        referral.set_ref(&mut deps.storage, &j, &a).unwrap();

        let all_ref_a = referral
            .all_referred_of(&deps.storage, a.clone(), None, None, None)
            .unwrap();

        assert_eq!(all_ref_a, vec![b.clone(), bb.clone(), h.clone(), j.clone()]);

        let all_ref_a = referral
            .all_referred_of(&deps.storage, a.clone(), Some(b.clone()), None, None)
            .unwrap();

        assert_eq!(all_ref_a, vec![bb.clone(), h.clone(), j.clone()]);

        let all_ref_a = referral
            .all_referred_of(&deps.storage, a.clone(), None, None, Some(false))
            .unwrap();

        assert_eq!(all_ref_a, vec![j.clone(), h.clone(), bb.clone(), b.clone()]);

        let all_ref_a = referral
            .all_referred_of(&deps.storage, a.clone(), Some(j.clone()), None, Some(false))
            .unwrap();

        assert_eq!(all_ref_a, vec![h.clone(), bb.clone(), b.clone()]);
    }
}
