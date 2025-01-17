//use crate::txbuilder::primitives::{BoundedBytes, PlutusData};

pub use pallas_codec::utils::Int;
pub use pallas_primitives::{BigInt, BoundedBytes, Constr, MaybeIndefArray, PlutusData};

pub trait IntoData {
    fn into_data(&self) -> PlutusData;
}

#[macro_export]
macro_rules! constr {
    ($tag:expr, $($field:expr),*) => {
        {
            use $crate::txbuilder::plutus::{Constr, PlutusData, MaybeIndefArray};

            let inner = Constr {
                tag: 121 + $tag,
                any_constructor: None,
                fields: MaybeIndefArray::Def(vec![$($field.into_data()),*]),
            };

            PlutusData::Constr(inner)
        }
    };
}

impl<const N: usize> IntoData for [u8; N] {
    fn into_data(&self) -> PlutusData {
        PlutusData::BoundedBytes(BoundedBytes::from(self.to_vec()))
    }
}

impl IntoData for Vec<u8> {
    fn into_data(&self) -> PlutusData {
        PlutusData::BoundedBytes(BoundedBytes::from(self.clone()))
    }
}

impl IntoData for u64 {
    fn into_data(&self) -> PlutusData {
        PlutusData::BigInt(BigInt::Int(Int::from(*self as i64)))
    }
}

mod test {
    use super::*;

    fn construct_constr() {
        let x = constr!(0, b"abc", vec![1, 2, 3]);
    }
}
