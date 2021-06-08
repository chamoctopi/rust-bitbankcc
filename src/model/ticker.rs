#[derive(Debug)]
pub struct Ticker {
    pub sell: f64,
    pub buy: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub last: f64,
    pub volume: f64,
    pub timestamp: u64,
}
