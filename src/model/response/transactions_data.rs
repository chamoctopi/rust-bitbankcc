use crate::model::response::Response;
use crate::model::{Transactions, TransactionsValue};
use crate::{Error, OrderSide};
use serde::Deserialize;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct TransactionsData {
    transactions: Vec<TransactionsInnerData>,
}

#[derive(Deserialize)]
pub struct TransactionsInnerData {
    transaction_id: u64,
    side: String,
    price: String,
    amount: String,
    executed_at: u64,
}

impl Into<Transactions> for TransactionsData {
    fn into(self) -> Transactions {
        let data = self.transactions;
        let mut values: Vec<TransactionsValue> = Vec::with_capacity(data.len());
        for dt in data {
            values.push(TransactionsValue {
                transaction_id: dt.transaction_id,
                side: OrderSide::from_str(&dt.side).unwrap(),
                price: dt.price.parse().unwrap(),
                amount: dt.amount.parse().unwrap(),
                executed_at: dt.executed_at,
            })
        }
        Transactions { values }
    }
}

impl TryFrom<Response> for TransactionsData {
    type Error = Error;

    fn try_from(resp: Response) -> Result<Self, Self::Error> {
        let code = resp.data.as_object().unwrap().get("code");
        if code.is_some() {
            return Err(Error::BitbankError(code.unwrap().as_i64().unwrap()));
        }
        Ok(serde_json::from_value::<Self>(resp.data)?)
    }
}
