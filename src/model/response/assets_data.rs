use crate::model::assets::*;
use crate::model::response::Response;
use crate::{BitbankError, Error};
use serde::Deserialize;
use serde_json::Value;
use std::convert::TryFrom;

#[derive(Deserialize)]
pub struct AssetsData {
    assets: Vec<AssetsInnerData>,
}

#[derive(Deserialize)]
struct AssetsInnerData {
    asset: String,
    free_amount: String,
    amount_precision: u8,
    onhand_amount: String,
    locked_amount: String,
    withdrawal_fee: Value,
    stop_deposit: bool,
    stop_withdrawal: bool,
}

impl Into<Assets> for AssetsData {
    fn into(self) -> Assets {
        let mut values: Vec<AssetsValue> = Vec::with_capacity(self.assets.len());
        for inner in self.assets {
            let withdrawal_fee = if inner.withdrawal_fee.is_object() {
                let obj = inner.withdrawal_fee.as_object().unwrap();
                AssetsValueWithdrawalFee::WithdrawalFeeObj(WithdrawalFeeObject {
                    threshold: obj
                        .get("threshold")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .parse()
                        .unwrap(),
                    under: obj.get("under").unwrap().as_str().unwrap().parse().unwrap(),
                    over: obj.get("over").unwrap().as_str().unwrap().parse().unwrap(),
                })
            } else {
                AssetsValueWithdrawalFee::WithdrawalFee(
                    inner.withdrawal_fee.as_str().unwrap().parse().unwrap(),
                )
            };
            values.push(AssetsValue {
                asset: inner.asset.clone(),
                free_amount: inner.free_amount.parse().unwrap(),
                amount_precision: inner.amount_precision,
                onhand_amount: inner.onhand_amount.parse().unwrap(),
                locked_amount: inner.locked_amount.parse().unwrap(),
                withdrawal_fee,
                stop_deposit: inner.stop_deposit,
                stop_withdrawal: inner.stop_withdrawal,
            })
        }
        Assets { values }
    }
}

impl TryFrom<Response> for AssetsData {
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
