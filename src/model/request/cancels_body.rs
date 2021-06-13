use crate::CurrencyPair;
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub struct CancelsBody {
    pair: CurrencyPair,
    order_ids: Vec<u64>,
}

impl CancelsBody {
    pub fn new(pair: CurrencyPair, order_ids: Vec<u64>) -> Self {
        Self { pair, order_ids }
    }
}

impl fmt::Display for CancelsBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("failed json serializing")
        )
    }
}
