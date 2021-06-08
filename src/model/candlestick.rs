use crate::model::enums::CandleType;

#[derive(Debug)]
pub struct Candlestick {
    pub r#type: CandleType,
    pub values: Vec<CandlestickValue>,
}

#[derive(Debug)]
pub struct CandlestickValue {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub timestamp: u64,
}
