use std::str::FromStr;
use strum;
use strum_macros::{Display, EnumString};

#[derive(Debug, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum CurrencyPair {
    BtcJpy,
    EthJpy,
}

#[test]
fn test_currency_pair() {
    // string to enum
    let pair = CurrencyPair::from_str("btc_jpy").unwrap();
    assert_eq!(String::from("btc_jpy"), format!("{}", pair));
    // enum to string
    let pair = CurrencyPair::BtcJpy.to_string();
    assert_eq!(String::from("btc_jpy"), pair);
}

#[derive(Debug, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum CandleType {
    _1min,
    _5min,
    _15min,
    _30min,
    _1hour,
    _4hour,
    _8hour,
    _12hour,
    _1day,
    _1week,
    _1month,
}

#[test]
fn test_candle_type() {
    // string to enum
    let typ = CandleType::from_str("1min").unwrap();
    assert_eq!(String::from("1min"), format!("{}", typ));
    // enum to string
    let typ = CandleType::_1min.to_string();
    assert_eq!(String::from("1min"), typ);
}
