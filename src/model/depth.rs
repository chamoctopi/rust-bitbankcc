#[derive(Debug)]
pub struct Depth {
    pub asks: Vec<DepthValue>,
    pub bids: Vec<DepthValue>,
}

#[derive(Debug)]
pub struct DepthValue {
    pub price: f64,
    pub amount: f64,
}
