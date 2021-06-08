use crate::model::ticker::*;
use serde::Deserialize;
use serde_json::Number;

#[derive(Deserialize)]
pub struct TickerResponse {
    success: u8,
    data: TickerData,
}

#[derive(Deserialize)]
struct TickerData {
    sell: String,
    buy: String,
    high: String,
    low: String,
    open: String,
    last: String,
    vol: String,
    timestamp: Number,
}

impl Into<Ticker> for TickerResponse {
    fn into(self) -> Ticker {
        let data = self.data;
        Ticker {
            sell: data.sell.parse().unwrap(),
            buy: data.buy.parse().unwrap(),
            high: data.high.parse().unwrap(),
            low: data.low.parse().unwrap(),
            open: data.open.parse().unwrap(),
            last: data.last.parse().unwrap(),
            volume: data.vol.parse().unwrap(),
            timestamp: data.timestamp.as_u64().unwrap(),
        }
    }
}
