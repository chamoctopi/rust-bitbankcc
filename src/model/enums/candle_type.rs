use std::str::FromStr; // DO NOT DELETE: this is used import
use strum_macros::{Display, EnumString};

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
fn string_to_enum() {
    matches!(CandleType::from_str("1min").unwrap(), CandleType::_1min,);
}

#[test]
fn enum_to_string() {
    assert_eq!(CandleType::_1min.to_string(), String::from("1min"));
}

#[test]
fn display_enum() {
    assert_eq!(format!("{}", CandleType::_1min), String::from("1min"));
}
