use crate::model::response::Response;
use crate::model::{ExchangeStatus, ExchangeStatusValue};
use crate::{BitbankError, CurrencyPair, Error};
use serde::Deserialize;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct ExchangeStatusData {
    statuses: Vec<ExchangeStatusInnerData>,
}

#[derive(Deserialize)]
struct ExchangeStatusInnerData {
    pair: String,
    status: String,
    min_amount: String,
}

impl Into<ExchangeStatus> for ExchangeStatusData {
    fn into(self) -> ExchangeStatus {
        let inner = self.statuses;
        let mut values: Vec<ExchangeStatusValue> = Vec::with_capacity(inner.len());
        for status in inner {
            values.push(ExchangeStatusValue {
                pair: CurrencyPair::from_str(&status.pair).unwrap(),
                status: status.status,
                min_amount: status.min_amount.parse().unwrap(),
            })
        }
        ExchangeStatus { values }
    }
}

impl TryFrom<Response> for ExchangeStatusData {
    type Error = Error;

    fn try_from(resp: Response) -> Result<Self, Self::Error> {
        let code = resp.data.as_object().unwrap().get("code");
        if code.is_some() {
            return Err(Error::ApiError(BitbankError::new(
                code.unwrap().as_i64().unwrap(),
            )));
        }
        Ok(serde_json::from_value::<Self>(resp.data)?)
    }
}
