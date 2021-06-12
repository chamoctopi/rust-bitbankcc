use crate::{CurrencyPair, OrderSide, OrderStatus, OrderType};

#[derive(Debug)]
pub struct Order {
    pub order_id: u64,
    pub pair: CurrencyPair,
    pub side: OrderSide,
    pub r#type: OrderType,
    pub start_amount: f64,
    pub remaining_amount: f64,
    pub executed_amount: f64,
    pub price: f64,
    pub post_only: bool,
    pub average_price: f64,
    pub ordered_at: u64,
    pub expire_at: u64,
    pub status: OrderStatus,
}
