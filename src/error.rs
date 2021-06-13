use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("bitbank error {0:?}")]
    ApiError(BitbankError),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    JsonParseError(#[from] serde_json::Error),
    #[error(transparent)]
    HttpError(#[from] http::Error),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
}

#[derive(Debug)]
pub struct BitbankError {
    code: i64,
    message: &'static str,
}

impl BitbankError {
    pub fn new(code: i64) -> Self {
        Self {
            code,
            message: match BITBANK_ERROR_CODE.get(&code) {
                Some(message) => message,
                None => "Unknown error",
            },
        }
    }
}

static BITBANK_ERROR_CODE: Lazy<HashMap<i64, &str>> = Lazy::new(|| {
    [
        (10000, "Url not found"),
        (10001, "System error"),
        (10002, "Malformed request"),
        (10003, "System error"),
        (10005, "Timeout waiting for response"),
        (20001, "Authentication failed api authorization"),
        (20002, "Invalid api key"),
        (20003, "Api key not found"),
        (20004, "Invalid api nonce"),
        (20005, "Invalid api signature"),
        (20011, "MFA failed"),
        (20014, "SMS verification failed"),
        (20023, "Missing OTP code"),
        (20024, "Missing SMS code"),
        (20025, "Missing OTP and SMS code"),
        (
            20026,
            "MFA is temporarily locked because too many failures. Please retry after 60 seconds",
        ),
        (30001, "Missing order quantity"),
        (30006, "Missing order id"),
        (30007, "Missing order id array"),
        (30009, "Missing asset"),
        (30012, "Missing order price"),
        (30013, "Missing side"),
        (30015, "Missing order type"),
        (30016, "Missing asset"),
        (30019, "Missing uuid"),
        (30039, "Missing withdraw amount"),
        (40001, "Invalid order quantity"),
        (40006, "Invalid count"),
        (40007, "Invalid end param"),
        (40008, "Invalid end_id"),
        (40009, "Invalid from_id"),
        (40013, "Invalid order id"),
        (40014, "Invalid order id array"),
        (40015, "Too many orders are specified"),
        (40017, "Invalid asset"),
        (40020, "Invalid order price"),
        (40021, "Invalid order side"),
        (40022, "Invalid trading start time"),
        (40024, "Invalid order type"),
        (40025, "Invalid asset"),
        (40028, "Invalid uuid"),
        (40048, "Invalid withdraw amount"),
        (40113, "Invalid post_only"),
        (40114, "post_only can not be specified with such order type"),
        (50003, "Account is restricted"),
        (50004, "Account is provisional"),
        (50005, "Account is blocked"),
        (50006, "Account is blocked"),
        (50008, "Identity verification is not finished"),
        (50009, "Order not found"),
        (50010, "Order can not be canceled"),
        (50011, "Api not found"),
        (50026, "Order has already been canceled"),
        (50027, "Order has already been executed"),
        (60001, "Insufficient amount"),
        (
            60002,
            "Market buy order quantity has exceeded the upper limit",
        ),
        (60003, "Order quantity has exceeded the limit"),
        (60004, "Order quantity has exceeded the lower threshold"),
        (60005, "Order quantity has exceeded the upper limit"),
        (60006, "Order price has exceeded the lower limit"),
        (60011, "Too many Simultaneous orders, current limit is 30"),
        (70001, "System error"),
        (70002, "System error"),
        (70003, "System error"),
        (
            70004,
            "Order is restricted during suspension of transactions",
        ),
        (70005, "Buy order has been temporarily restricted"),
        (70006, "Sell order has been temporarily restricted"),
        (
            70009,
            "Market order has been temporarily restricted. Please use limit order instead",
        ),
        (70010, "Minimum Order Quantity is increased temporarily"),
        (70011, "System is busy. Please try again"),
        (70012, "System error"),
        (70013, "Order and cancel has been temporarily restricted"),
        (
            70014,
            "Withdraw and cancel request has been temporarily restricted",
        ),
        (
            70015,
            "Lending and cancel request has been temporarily restricted",
        ),
        (70016, "Lending and cancel request has been restricted"),
        (70017, "Orders on pair have been suspended"),
        (70018, "Order and cancel on pair have been suspended"),
        (70019, "Order cancel request is in process"),
        (70020, "Market order has been temporarily restricted"),
        (70021, "Limit order price is over the threshold"),
    ]
    .iter()
    .cloned()
    .collect()
});
