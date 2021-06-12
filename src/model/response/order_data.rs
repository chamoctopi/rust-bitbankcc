use crate::model::order::*;
use crate::model::response::Response;
use crate::{CurrencyPair, Error, OrderSide, OrderStatus, OrderType};
use serde::Deserialize;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct OrderData {
    order_id: u64,
    pair: String,
    side: String,
    r#type: String,
    start_amount: String,
    remaining_amount: String,
    executed_amount: String,
    price: String,
    post_only: bool,
    average_price: String,
    ordered_at: u64,
    expire_at: u64,
    status: String,
}

impl Into<Order> for OrderData {
    fn into(self) -> Order {
        Order {
            order_id: self.order_id,
            pair: CurrencyPair::from_str(&self.pair).unwrap(),
            side: OrderSide::from_str(&self.side).unwrap(),
            r#type: OrderType::from_str(&self.r#type).unwrap(),
            start_amount: self.start_amount.parse().unwrap(),
            remaining_amount: self.remaining_amount.parse().unwrap(),
            executed_amount: self.executed_amount.parse().unwrap(),
            price: self.price.parse().unwrap(),
            post_only: self.post_only,
            average_price: self.average_price.parse().unwrap(),
            ordered_at: self.ordered_at,
            expire_at: self.expire_at,
            status: OrderStatus::from_str(&self.status).unwrap(),
        }
    }
}

impl TryFrom<Response> for OrderData {
    type Error = Error;

    fn try_from(resp: Response) -> Result<Self, Self::Error> {
        let code = resp.data.as_object().unwrap().get("code");
        if code.is_some() {
            return Err(Self::Error::Code(code.unwrap().as_i64().unwrap()));
        }
        Ok(serde_json::from_value::<Self>(resp.data)?)
    }
}
