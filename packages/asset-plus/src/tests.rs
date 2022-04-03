mod tests {
    use cosmwasm_std::{testing::mock_dependencies, Addr};
    use cw_storage_plus::Map;

    use crate::{Asset, AssetInfo};

    #[test]
    fn test_eq() {
        let info1 = AssetInfo::Token {
            contract_addr: Addr::unchecked("1"),
        };

        let info11 = AssetInfo::Token {
            contract_addr: Addr::unchecked("1"),
        };

        let info2 = AssetInfo::Token {
            contract_addr: Addr::unchecked("2"),
        };

        assert_eq!(info1, info11);
        assert_ne!(info1, info2);

        let a1 = Asset::new(info1, 10000u128);
        let a11 = Asset::new(info11, 10000u128);
        let a2 = Asset::new(info2, 10000u128);

        assert_eq!(a1, a11);
        assert_ne!(a1, a2);
    }

    #[test]
    fn arithmethic() {
        let mut a1 = Asset::default();

        a1 += 100u64;

        assert_eq!(a1.amount, 100u64.into());

        let mut a2 = a1 + 200u64;

        assert_eq!(a2.amount, 300u64.into());

        a2 -= 100u64;

        assert_eq!(a2.amount, 200u64.into());

        let mut a3 = a2 - 100u64;

        assert_eq!(a3.amount, 100u64.into());

        a3 *= 5u64;

        assert_eq!(a3.amount, 500u64.into());

        let mut a4 = a3 * 2u64;

        assert_eq!(a4.amount, 1000u64.into());

        a4 /= 2u64;

        assert_eq!(a4.amount, 500u64.into());

        let mut a5 = a4 / 5u64;

        assert_eq!(a5.amount, 100u64.into());

        a5 %= 70u64;

        assert_eq!(a5.amount, 30u64.into());

        let a6 = a5 % 29u64;

        assert_eq!(a6.amount, 1u64.into());
    }

    #[test]
    fn test_map_key() {
        let mut deps = mock_dependencies(&[]);

        let map_ref: Map<&AssetInfo, u64> = Map::new("something_ref");
        let map: Map<AssetInfo, u64> = Map::new("something");

        let info1 = AssetInfo::Token {
            contract_addr: Addr::unchecked("1"),
        };

        let info2 = AssetInfo::Token {
            contract_addr: Addr::unchecked("2"),
        };

        let info3 = AssetInfo::NativeToken {
            denom: "uusd".into(),
        };

        map_ref.save(&mut deps.storage, &info1, &100).unwrap();
        map_ref.save(&mut deps.storage, &info2, &200).unwrap();
        map_ref.save(&mut deps.storage, &info3, &300).unwrap();

        assert_eq!(map_ref.load(&deps.storage, &info1).unwrap(), 100);
        assert_eq!(map_ref.load(&deps.storage, &info2).unwrap(), 200);
        assert_eq!(map_ref.load(&deps.storage, &info3).unwrap(), 300);

        map.save(&mut deps.storage, info1.clone(), &100).unwrap();
        map.save(&mut deps.storage, info2.clone(), &200).unwrap();
        map.save(&mut deps.storage, info3.clone(), &300).unwrap();

        assert_eq!(map.load(&deps.storage, info1).unwrap(), 100);
        assert_eq!(map.load(&deps.storage, info2).unwrap(), 200);
        assert_eq!(map.load(&deps.storage, info3).unwrap(), 300);
    }
}
