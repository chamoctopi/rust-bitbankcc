#[derive(Debug)]
pub struct Assets {
    pub values: Vec<AssetsValue>,
}

#[derive(Debug)]
pub struct AssetsValue {
    pub asset: String,
    pub free_amount: f64,
    pub amount_precision: u8,
    pub onhand_amount: f64,
    pub locked_amount: f64,
    pub withdrawal_fee: AssetsValueWithdrawalFee,
    pub stop_deposit: bool,
    pub stop_withdrawal: bool,
}

#[derive(Debug)]
pub enum AssetsValueWithdrawalFee {
    WithdrawalFee(f64),
    WithdrawalFeeObj(WithdrawalFeeObject),
}

#[derive(Debug)]
pub struct WithdrawalFeeObject {
    pub threshold: f64,
    pub under: f64,
    pub over: f64,
}
