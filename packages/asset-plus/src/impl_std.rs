use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

use cosmwasm_std::Uint128;

use crate::asset::{Asset, AssetInfo};

impl Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.info, self.amount)
    }
}

impl Display for AssetInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetInfo::Token { contract_addr } => {
                write!(f, "Token at {}", contract_addr)
            }
            AssetInfo::NativeToken { denom } => write!(f, "Native token with denom {}", denom),
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
