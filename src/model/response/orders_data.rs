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
        let orders_data = self.orders;
        let mut orders: Vec<Order> = Vec::with_capacity(orders_data.len());
        for order_data in orders_data {
            orders.push(order_data.into());
        }
        Orders { orders }
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
