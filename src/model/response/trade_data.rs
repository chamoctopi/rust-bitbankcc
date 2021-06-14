use crate::model::response::Response;
use crate::model::{Trade, TradeValue};
use crate::{BitbankError, CurrencyPair, Error, OrderSide, OrderType};
use serde::Deserialize;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct TradeData {
    trades: Vec<TradeInnerData>,
}

#[derive(Deserialize)]
struct TradeInnerData {
    trade_id: u64,
    pair: String,
    order_id: u64,
    side: String,
    r#type: String,
    amount: String,
    price: String,
    maker_taker: String,
    fee_amount_base: String,
    fee_amount_quote: String,
    executed_at: u64,
}

impl Into<Trade> for TradeData {
    fn into(self) -> Trade {
        let inner = self.trades;
        let mut values: Vec<TradeValue> = Vec::with_capacity(inner.len());
        for trade in inner {
            values.push(TradeValue {
                trade_id: trade.trade_id,
                pair: CurrencyPair::from_str(&trade.pair).unwrap(),
                order_id: trade.order_id,
                side: OrderSide::from_str(&trade.side).unwrap(),
                r#type: OrderType::from_str(&trade.r#type).unwrap(),
                amount: trade.amount.parse().unwrap(),
                price: trade.price.parse().unwrap(),
                maker_taker: trade.maker_taker,
                fee_amount_base: trade.fee_amount_base.parse().unwrap(),
                fee_amount_quote: trade.fee_amount_quote.parse().unwrap(),
                executed_at: trade.executed_at,
            })
        }
        Trade { values }
    }
}

impl TryFrom<Response> for TradeData {
    type Error = Error;

    fn try_from(resp: Response) -> Result<Self, Self::Error> {
        let code = resp.data.as_object().unwrap().get("code");
        if code.is_some() {
            return Err(Error::ApiError(BitbankError::new(
                code.unwrap().as_i64().unwrap(),
            )));
        }
        Ok(serde_json::from_value::<Self>(resp.data)?)
    }
}
