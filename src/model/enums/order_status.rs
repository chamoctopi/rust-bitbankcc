use serde_with::SerializeDisplay;
use std::str::FromStr; // DO NOT DELETE: this is used import
use strum_macros::{Display, EnumString};

#[derive(Debug, Display, EnumString, SerializeDisplay)]
#[strum(serialize_all = "snake_case")]
pub enum OrderStatus {
    Unfilled,
    PartiallyFilled,
    FullyFilled,
    CanceledUnfilled,
    CanceledPartiallyFilled,
}

#[test]
fn string_to_enum() {
    matches!(
        OrderStatus::from_str("fully_filled").unwrap(),
        OrderStatus::FullyFilled,
    );
}

#[test]
fn enum_to_string() {
    assert_eq!(
        OrderStatus::FullyFilled.to_string(),
        String::from("fully_filled")
    );
}

#[test]
fn display_enum() {
    assert_eq!(
        format!("{}", OrderStatus::FullyFilled),
        String::from("fully_filled")
    );
}
