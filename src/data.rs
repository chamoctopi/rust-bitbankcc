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

#[derive(Debug)]
pub struct Assets {
    pub values: Vec<AssetsValue>,
}

#[derive(Debug)]
pub struct AssetsValue {
    pub asset: String,
    pub free_amount: f64,
    pub amount_precision: u8,
    pub onhand_amount: f64,
    pub locked_amount: f64,
    pub withdrawal_fee: AssetsValueWithdrawalFee,
    pub stop_deposit: bool,
    pub stop_withdrawal: bool,
}

#[derive(Debug)]
pub enum AssetsValueWithdrawalFee {
    WithdrawalFee(f64),
    WithdrawalFeeObj(WithdrawalFeeObject),
}

#[derive(Debug)]
pub struct WithdrawalFeeObject {
    pub threshold: f64,
    pub under: f64,
    pub over: f64,
}
