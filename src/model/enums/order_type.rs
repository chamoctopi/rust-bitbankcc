use std::str::FromStr; // DO NOT DELETE: this is used import
use strum_macros::{Display, EnumString};

#[derive(Debug, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum OrderType {
    Limit,
    Market,
}

#[test]
fn string_to_enum() {
    matches!(OrderType::from_str("market").unwrap(), OrderType::Market,);
}

#[test]
fn enum_to_string() {
    assert_eq!(OrderType::Market.to_string(), String::from("market"));
}

#[test]
fn display_enum() {
    assert_eq!(format!("{}", OrderType::Market), String::from("market"));
}