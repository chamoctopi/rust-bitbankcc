use crate::model::response::Response;
use crate::model::ticker::*;
use crate::MyError;
use serde::Deserialize;
use serde_json::Number;
use std::convert::TryFrom;

#[derive(Deserialize)]
pub struct TickerData {
    sell: String,
    buy: String,
    high: String,
    low: String,
    open: String,
    last: String,
    vol: String,
    timestamp: u64,
}

impl Into<Ticker> for TickerData {
    fn into(self) -> Ticker {
        Ticker {
            sell: self.sell.parse().unwrap(),
            buy: self.buy.parse().unwrap(),
            high: self.high.parse().unwrap(),
            low: self.low.parse().unwrap(),
            open: self.open.parse().unwrap(),
            last: self.last.parse().unwrap(),
            volume: self.vol.parse().unwrap(),
            timestamp: self.timestamp,
        }
    }
}

impl TryFrom<Response> for TickerData {
    type Error = MyError;

    fn try_from(resp: Response) -> Result<Self, Self::Error> {
        let code = resp.data.as_object().unwrap().get("code");
        if code.is_some() {
            return Err(Self::Error::Code(code.unwrap().as_i64().unwrap()));
        }
        Ok(serde_json::from_value::<Self>(resp.data)?)
    }
}
