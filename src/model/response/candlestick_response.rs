use crate::model::candlestick::*;
use crate::model::enums::CandleType;
use serde::Deserialize;
use serde_json::Number;
use std::str::FromStr;

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
