use serde_with::SerializeDisplay;
use strum_macros::{Display, EnumString};

#[derive(Debug, Display, EnumString, SerializeDisplay)]
#[strum(serialize_all = "snake_case")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[cfg(test)]
mod tests {
    use crate::OrderSide;
    use std::str::FromStr;

    #[test]
    fn string_to_enum() {
        matches!(OrderSide::from_str("buy").unwrap(), OrderSide::Buy);
    }

    #[test]
    fn enum_to_string() {
        assert_eq!(OrderSide::Buy.to_string(), String::from("buy"));
    }

    #[test]
    fn display_enum() {
        assert_eq!(format!("{}", OrderSide::Buy), String::from("buy"));
    }
}
