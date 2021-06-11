use crate::{CurrencyPair, OrderSide, OrderType};

pub struct OrderBody {
    pub pair: CurrencyPair,
    pub amount: f64,
    pub price: f64,
    pub side: OrderSide,
    pub r#type: OrderType,
    pub post_only: bool,
}
