use crate::CurrencyPair;

#[derive(Debug)]
pub struct ExchangeStatus {
    pub values: Vec<ExchangeStatusValue>,
}

#[derive(Debug)]
pub struct ExchangeStatusValue {
    pub pair: CurrencyPair,
    pub status: String,
    pub min_amount: f64,
}
