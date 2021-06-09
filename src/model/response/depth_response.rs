use crate::model::depth::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DepthResponse {
    success: u8,
    data: DepthData,
}

#[derive(Deserialize)]
struct DepthData {
    asks: Vec<(String, String)>,
    bids: Vec<(String, String)>,
}

impl Into<Depth> for DepthResponse {
    fn into(self) -> Depth {
        let data = self.data;
        let mut asks: Vec<DepthValue> = Vec::with_capacity(data.asks.len());
        let mut bids: Vec<DepthValue> = Vec::with_capacity(data.bids.len());
        for ask in data.asks {
            let price = ask.0.parse().unwrap();
            let amount = ask.1.parse().unwrap();
            asks.push(DepthValue { price, amount });
        }
        for bid in data.bids {
            let price = bid.0.parse().unwrap();
            let amount = bid.1.parse().unwrap();
            bids.push(DepthValue { price, amount });
        }
        Depth { asks, bids }
    }
}
