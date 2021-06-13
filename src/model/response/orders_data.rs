use crate::model::response::{OrderData, Response};
use crate::model::{Order, Orders};
use crate::Error;
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Deserialize)]
pub struct OrdersData {
    orders: Vec<OrderData>,
}

impl Into<Orders> for OrdersData {
    fn into(self) -> Orders {
        let orders = self.orders;
        let mut values: Vec<Order> = Vec::with_capacity(orders.len());
        for order in orders {
            values.push(order.into());
        }
        Orders { values }
    }
}

impl TryFrom<Response> for OrdersData {
    type Error = Error;

    fn try_from(resp: Response) -> Result<Self, Self::Error> {
        let code = resp.data.as_object().unwrap().get("code");
        if code.is_some() {
            return Err(Error::BitbankError(code.unwrap().as_i64().unwrap()));
        }
        Ok(serde_json::from_value::<Self>(resp.data)?)
    }
}
