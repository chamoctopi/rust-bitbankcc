use crate::{CurrencyPair, OrderSide, OrderType};
use serde::Serialize;
// use serde::{Serialize, Serializer};
// use serde_with::{serde_as, SerializeAs};
use std::fmt;

// #[serde_as]
#[derive(Serialize)]
pub struct OrderBody {
    pair: CurrencyPair,
    // #[serde_as(as = "FloatFromString")]
    amount: f64,
    // #[serde_as(as = "FloatFromString")]
    price: f64,
    side: OrderSide,
    r#type: OrderType,
    post_only: bool,
}

impl OrderBody {
    pub fn new(
        pair: CurrencyPair,
        price: f64,
        amount: f64,
        side: OrderSide,
        r#type: OrderType,
        post_only: bool,
    ) -> Self {
        Self {
            pair,
            price,
            amount,
            side,
            r#type,
            post_only,
        }
    }
}

impl fmt::Display for OrderBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("failed json serializing")
        )
    }
}

// struct FloatFromString {}
//
// impl SerializeAs<f64> for FloatFromString {
//     fn serialize_as<S>(source: &f64, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_str(&source.to_string())
//     }
// }
