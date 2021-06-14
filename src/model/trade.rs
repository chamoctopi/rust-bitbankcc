use crate::{CurrencyPair, OrderSide, OrderType};

#[derive(Debug)]
pub struct Trade {
    pub values: Vec<TradeValue>,
}

#[derive(Debug)]
pub struct TradeValue {
    pub trade_id: u64,
    pub pair: CurrencyPair,
    pub order_id: u64,
    pub side: OrderSide,
    pub r#type: OrderType,
    pub amount: f64,
    pub price: f64,
    pub maker_taker: String,
    pub fee_amount_base: f64,
    pub fee_amount_quote: f64,
    pub executed_at: u64,
}
