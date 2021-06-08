use super::data::*;
use super::enums::*;
use super::request::*;
use super::response::*;
use http::uri;
use serde::Deserialize;

const ENDPOINT_PUBLIC: &str = "public.bitbank.cc";
const ENDPOINT_PRIVATE: &str = "api.bitbank.cc";

pub struct PublicClient {
    pair: CurrencyPair,
}

impl PublicClient {
    pub fn new(pair: CurrencyPair) -> PublicClient {
        PublicClient { pair }
    }
}

impl PublicClient {
    #[tokio::main]
    pub async fn get_ticker(&self) -> Result<Ticker, Box<dyn std::error::Error>> {
        let path = format!("/{}/ticker", &self.pair.to_string());
        let builder = get_public_uri_builder(path);
        let resp = reqwest::get(builder.build().unwrap().to_string())
            .await?
            .json::<TickerResponse>()
            .await?;
        Ok(resp.into())
    }

    #[tokio::main]
    pub async fn get_depth(&self) -> Result<Depth, Box<dyn std::error::Error>> {
        let path = format!("/{}/depth", &self.pair.to_string());
        let builder = get_public_uri_builder(path);
        let resp = reqwest::get(builder.build().unwrap().to_string())
            .await?
            .json::<DepthResponse>()
            .await?;
        Ok(resp.into())
    }

    #[tokio::main]
    pub async fn get_candlestick(
        &self,
        r#type: CandleType,
        YYYMMDD: String,
    ) -> Result<Candlestick, Box<dyn std::error::Error>> {
        let path = format!(
            "/{}/candlestick/{}/{}",
            &self.pair.to_string(),
            &r#type.to_string(),
            YYYMMDD
        );
        let builder = get_public_uri_builder(path);
        let resp = reqwest::get(builder.build().unwrap().to_string())
            .await?
            .json::<CandlestickResponse>()
            .await?;
        Ok(resp.into())
    }
}

fn get_public_uri_builder(path: String) -> uri::Builder {
    uri::Builder::new()
        .scheme("https")
        .authority(ENDPOINT_PUBLIC)
        .path_and_query(path)
}

// async fn doHttpGet<R, D>(builder: uri::Builder) -> Result<D, Box<dyn std::error::Error>>
// where
//     R: Deserialize,
//     D: Into<D>,
// {
//     let uri = builder.build().unwrap();
//     let resp = reqwest::get(uri.to_string()).await?.json::<R>().await?;
//     Ok(resp.into())
// }
