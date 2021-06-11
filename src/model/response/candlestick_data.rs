use crate::model::candlestick::*;
use crate::model::enums::CandleType;
use crate::model::response::Response;
use crate::MyError;
use serde::Deserialize;
use serde_json::Number;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct CandlestickData {
    candlestick: Vec<CandlestickInnerData>,
}

#[derive(Deserialize)]
struct CandlestickInnerData {
    r#type: String,
    ohlcv: Vec<(String, String, String, String, String, Number)>,
}

impl Into<Candlestick> for CandlestickData {
    fn into(self) -> Candlestick {
        let inner = &self.candlestick[0];
        let mut values: Vec<CandlestickValue> = Vec::with_capacity(inner.ohlcv.len());
        for ohlcv in &inner.ohlcv {
            values.push(CandlestickValue {
                open: ohlcv.0.parse().unwrap(),
                high: ohlcv.1.parse().unwrap(),
                low: ohlcv.2.parse().unwrap(),
                close: ohlcv.3.parse().unwrap(),
                volume: ohlcv.4.parse().unwrap(),
                timestamp: ohlcv.5.as_u64().unwrap(),
            })
        }
        Candlestick {
            r#type: CandleType::from_str(inner.r#type.as_str()).unwrap(),
            values,
        }
    }
}

impl TryFrom<Response> for CandlestickData {
    type Error = MyError;

    fn try_from(resp: Response) -> Result<Self, Self::Error> {
        let code = resp.data.as_object().unwrap().get("code");
        if code.is_some() {
            return Err(Self::Error::Code(code.unwrap().as_i64().unwrap()));
        }
        Ok(serde_json::from_value::<Self>(resp.data)?)
    }
}
