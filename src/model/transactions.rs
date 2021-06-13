use crate::OrderSide;

#[derive(Debug)]
pub struct Transactions {
    pub values: Vec<TransactionsValue>,
}

#[derive(Debug)]
pub struct TransactionsValue {
    pub transaction_id: u64,
    pub side: OrderSide,
    pub price: f64,
    pub amount: f64,
    pub executed_at: u64,
}
