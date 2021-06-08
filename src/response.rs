use super::data::*;
use super::enums::CandleType;
use serde::Deserialize;
use serde_json::Number;
use std::str::FromStr;

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

#[derive(Deserialize)]
pub struct DepthResponse {
    success: u8,
    data: DepthData,
}

#[derive(Deserialize)]
struct DepthData {
    asks: Vec<(String, String)>,
    bids: Vec<(String, String)>,
}

impl Into<Depth> for DepthResponse {
    fn into(self) -> Depth {
        let data = self.data;
        let mut asks: Vec<DepthValue> = Vec::with_capacity(data.asks.len());
        let mut bids: Vec<DepthValue> = Vec::with_capacity(data.bids.len());
        for ask in data.asks {
            let price = ask.0.parse().unwrap();
            let amount = ask.1.parse().unwrap();
            asks.push(DepthValue { price, amount });
        }
        for bid in data.bids {
            let price = bid.0.parse().unwrap();
            let amount = bid.1.parse().unwrap();
            bids.push(DepthValue { price, amount });
        }
        Depth { asks, bids }
    }
}

#[derive(Deserialize)]
pub struct CandlestickResponse {
    success: u8,
    data: CandlestickData,
}

#[derive(Deserialize)]
struct CandlestickData {
    candlestick: Vec<CandlestickInnerData>,
}

#[derive(Deserialize)]
struct CandlestickInnerData {
    r#type: String,
    ohlcv: Vec<(String, String, String, String, String, Number)>,
}

impl Into<Candlestick> for CandlestickResponse {
    fn into(self) -> Candlestick {
        let data = self.data;
        let inner = &data.candlestick[0];
        let mut values: Vec<CandlestickValue> = Vec::with_capacity(inner.ohlcv.len());
        for vals in &inner.ohlcv {
            values.push(CandlestickValue {
                open: vals.0.parse().unwrap(),
                high: vals.1.parse().unwrap(),
                low: vals.2.parse().unwrap(),
                close: vals.3.parse().unwrap(),
                volume: vals.4.parse().unwrap(),
                timestamp: vals.5.as_u64().unwrap(),
            })
        }
        Candlestick {
            r#type: CandleType::from_str(inner.r#type.as_str()).unwrap(),
            values,
        }
    }
}

#[derive(Deserialize)]
pub struct AssetsResponse {
    success: u8,
    data: AssetsData,
}

#[derive(Deserialize)]
struct AssetsData {
    assets: Vec<AssetsInnerData>,
}

#[derive(Deserialize)]
struct AssetsInnerData {
    asset: String,
    free_amount: String,
    amount_precision: Number,
    onhand_amount: String,
    locked_amount: String,
    withdrawal_fee: String,
    stop_deposit: bool,
    stop_withdrawal: bool,
}

impl Into<Assets> for AssetsResponse {
    fn into(self) -> Assets {
        let data = self.data;
        let mut values: Vec<AssetsValue> = Vec::with_capacity(data.assets.len());
        for val in &data.assets {
            values.push(AssetsValue {
                asset: "aho".to_string(),
                free_amount: 0.0,
                amount_precision: 0,
                onhand_amount: 0.0,
                locked_amount: 0.0,
                withdrawal_fee: 0.0,
                stop_deposit: false,
                stop_withdrawal: false,
            })
        }
        Assets { values }
    }
}
