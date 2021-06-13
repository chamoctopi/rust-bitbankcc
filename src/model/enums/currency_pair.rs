use serde_with::SerializeDisplay;
use strum_macros::{Display, EnumString};

#[derive(Debug, Display, EnumString, SerializeDisplay)]
#[strum(serialize_all = "snake_case")]
pub enum CurrencyPair {
    BtcJpy,
    XrpJpy,
    EthJpy,
    LtcJpy,
    BccJpy,
    MonaJpy,
    XlmJpy,
    QtumJpy,
    BatJpy,
    XrpBtc,
    EthBtc,
    LtcBtc,
    BccBtc,
    MonaBtc,
    XlmBtc,
    QtumBtc,
    BatBtc,
}

#[test]
fn string_to_enum() {
    matches!(
        CurrencyPair::from_str("btc_jpy").unwrap(),
        CurrencyPair::BtcJpy,
    );
}

#[test]
fn enum_to_string() {
    assert_eq!(CurrencyPair::BtcJpy.to_string(), String::from("btc_jpy"));
}

#[test]
fn display_enum() {
    assert_eq!(format!("{}", CurrencyPair::BtcJpy), String::from("btc_jpy"));
}
