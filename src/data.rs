use super::enums::*;

#[derive(Debug)]
pub struct Ticker {
    pub sell: f64,
    pub buy: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub last: f64,
    pub volume: f64,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct Depth {
    pub asks: Vec<DepthValue>,
    pub bids: Vec<DepthValue>,
}

#[derive(Debug)]
pub struct DepthValue {
    pub price: f64,
    pub amount: f64,
}

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
