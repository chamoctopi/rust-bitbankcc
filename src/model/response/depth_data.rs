use crate::model::depth::*;
use crate::model::response::Response;
use crate::MyError;
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Deserialize)]
pub struct DepthData {
    asks: Vec<(String, String)>,
    bids: Vec<(String, String)>,
}

impl Into<Depth> for DepthData {
    fn into(self) -> Depth {
        let mut asks: Vec<DepthValue> = Vec::with_capacity(self.asks.len());
        let mut bids: Vec<DepthValue> = Vec::with_capacity(self.bids.len());
        for ask in self.asks {
            let price = ask.0.parse().unwrap();
            let amount = ask.1.parse().unwrap();
            asks.push(DepthValue { price, amount });
        }
        for bid in self.bids {
            let price = bid.0.parse().unwrap();
            let amount = bid.1.parse().unwrap();
            bids.push(DepthValue { price, amount });
        }
        Depth { asks, bids }
    }
}

impl TryFrom<Response> for DepthData {
    type Error = MyError;

    fn try_from(resp: Response) -> Result<Self, Self::Error> {
        let code = resp.data.as_object().unwrap().get("code");
        if code.is_some() {
            return Err(Self::Error::Code(code.unwrap().as_i64().unwrap()));
        }
        Ok(serde_json::from_value::<Self>(resp.data)?)
    }
}
