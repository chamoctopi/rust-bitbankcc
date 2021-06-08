use crate::model::assets::*;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct AssetsResponse {
    success: u8,
    data: AssetsData,
}

#[derive(Deserialize)]
struct AssetsData {
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

impl Into<Assets> for AssetsResponse {
    fn into(self) -> Assets {
        let data = self.data;
        let mut values: Vec<AssetsValue> = Vec::with_capacity(data.assets.len());
        for val in &data.assets {
            let withdrawal_fee = if val.withdrawal_fee.is_object() {
                let obj = val.withdrawal_fee.as_object().unwrap();
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
                    val.withdrawal_fee.as_str().unwrap().parse().unwrap(),
                )
            };
            values.push(AssetsValue {
                asset: val.asset.as_str().to_string(),
                free_amount: val.free_amount.parse().unwrap(),
                amount_precision: val.amount_precision,
                onhand_amount: val.onhand_amount.parse().unwrap(),
                locked_amount: val.locked_amount.parse().unwrap(),
                withdrawal_fee,
                stop_deposit: val.stop_deposit,
                stop_withdrawal: val.stop_withdrawal,
            })
        }
        Assets { values }
    }
}
