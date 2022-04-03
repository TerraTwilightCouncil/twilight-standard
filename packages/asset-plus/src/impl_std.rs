use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

use cosmwasm_std::{Api, StdError, StdResult, Uint128};

use crate::asset::{Asset, AssetInfo};

impl Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.info, self.amount)
    }
}

impl Display for AssetInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Default for AssetInfo {
    fn default() -> Self {
        AssetInfo::NativeToken {
            denom: String::from("uusd"),
        }
    }
}

impl Sum for Asset {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Asset::default(), |asset, next| Asset {
            amount: next.amount + asset.amount,
            ..next
        })
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
}

impl Asset {
    pub fn new<A: Into<Uint128>>(info: AssetInfo, amount: A) -> Self {
        Asset {
            info,
            amount: amount.into(),
        }
    }
}

crate::oprt_impl!(add, Add, add_assign, AddAssign);
crate::oprt_impl!(sub, Sub, sub_assign, SubAssign);
crate::oprt_impl!(mul, Mul, mul_assign, MulAssign);
crate::oprt_impl!(div, Div, div_assign, DivAssign);
crate::oprt_impl!(rem, Rem, rem_assign, RemAssign);

#[macro_export]
macro_rules! oprt_impl {
    ($method: ident, $op: ident, $assign_method: ident, $op_assign: ident) => {
        impl<T> $op<T> for Asset
        where
            T: Into<Uint128>,
        {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: T) -> Self::Output {
                Asset {
                    amount: self.amount.$method(rhs.into()),
                    ..self
                }
            }
        }

        impl<T> $op<T> for &Asset
        where
            T: Into<Uint128>,
        {
            type Output = Asset;

            #[inline]
            fn $method(self, rhs: T) -> Self::Output {
                Asset {
                    amount: self.amount.$method(rhs.into()),
                    info: self.info.to_owned(),
                }
            }
        }

        impl $op for Asset {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: Self) -> Self::Output {
                assert!(
                    self.info == rhs.info,
                    "Attempt arithmetic operation with different info",
                );
                Asset {
                    amount: self.amount.$method(rhs.amount),
                    ..self
                }
            }
        }

        impl $op for &Asset {
            type Output = Asset;

            #[inline]
            fn $method(self, rhs: Self) -> Self::Output {
                assert!(
                    self.info == rhs.info,
                    "Attempt arithmetic operation with different info",
                );
                Asset {
                    amount: self.amount.$method(rhs.amount),
                    info: self.info.to_owned(),
                }
            }
        }

        impl $op<&Asset> for Asset {
            type Output = Asset;

            #[inline]
            fn $method(self, rhs: &Asset) -> Self::Output {
                assert!(
                    self.info == rhs.info,
                    "Attempt arithmetic operation with different info",
                );
                Asset {
                    amount: self.amount.$method(rhs.amount),
                    ..self
                }
            }
        }

        impl $op<Asset> for &Asset {
            type Output = Asset;

            #[inline]
            fn $method(self, rhs: Asset) -> Self::Output {
                assert!(
                    self.info == rhs.info,
                    "Attempt arithmetic operation with different info",
                );
                Asset {
                    amount: self.amount.$method(rhs.amount),
                    ..rhs
                }
            }
        }

        impl<T> $op_assign<T> for Asset
        where
            T: Into<Uint128>,
        {
            #[inline]
            fn $assign_method(&mut self, rhs: T) {
                self.amount.$assign_method(rhs.into())
            }
        }
    };
}
