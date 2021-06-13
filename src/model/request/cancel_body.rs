use crate::CurrencyPair;
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub struct CancelBody {
    pair: CurrencyPair,
    order_id: u64,
}

impl CancelBody {
    pub fn new(pair: CurrencyPair, order_id: u64) -> Self {
        Self { pair, order_id }
    }
}

impl fmt::Display for CancelBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("failed json serializing")
        )
    }
}
