use std::str::FromStr;

pub enum CurrencyPair {
    BtcJpy,
    EthJpy,
}

pub fn value_in_currency_pairs(pair: &CurrencyPair) -> &str {
    match pair {
        CurrencyPair::BtcJpy => "btc_jpy",
        CurrencyPair::EthJpy => "eth_jpy",
    }
}

#[derive(Debug)]
pub enum CandleType {
    _1Min,
    _5Min,
    _15Min,
    _30Min,
    _1Hour,
    _4Hour,
    _8Hour,
    _12Hour,
    _1Day,
    _1Week,
    _1Month,
}

impl FromStr for CandleType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1min" => Ok(CandleType::_1Min),
            "5min" => Ok(CandleType::_5Min),
            "15min" => Ok(CandleType::_15Min),
            "30min" => Ok(CandleType::_30Min),
            "1hour" => Ok(CandleType::_1Hour),
            "4hour" => Ok(CandleType::_4Hour),
            "8hour" => Ok(CandleType::_8Hour),
            "12hour" => Ok(CandleType::_12Hour),
            "1day" => Ok(CandleType::_1Day),
            "1week" => Ok(CandleType::_1Week),
            "1month" => Ok(CandleType::_1Month),
            _ => Err(()),
        }
    }
}

pub fn value_in_candle_types(r#type: &CandleType) -> &str {
    match r#type {
        CandleType::_1Min => "1min",
        CandleType::_5Min => "5min",
        CandleType::_15Min => "15min",
        CandleType::_30Min => "30min",
        CandleType::_1Hour => "1hour",
        CandleType::_4Hour => "4hour",
        CandleType::_8Hour => "8hour",
        CandleType::_12Hour => "12hour",
        CandleType::_1Day => "1day",
        CandleType::_1Week => "1week",
        CandleType::_1Month => "1month",
    }
}
