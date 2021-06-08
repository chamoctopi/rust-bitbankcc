use super::data::*;
use super::enums::*;
use super::request::*;
use super::response::*;
use hmac::{Hmac, Mac, NewMac};
use http::uri;
use serde::Deserialize;
use sha2::Sha256;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const ENDPOINT: &str = "api.bitbank.cc";

type HmacSha256 = Hmac<Sha256>;

pub struct PrivateClient {
    key: String,
    secret: String,
}

impl PrivateClient {
    pub fn new(key: String, secret: String) -> PrivateClient {
        PrivateClient { key, secret }
    }
}

impl PrivateClient {
    #[tokio::main]
    pub async fn get_ticker(&self) -> Result<Ticker, Box<dyn std::error::Error>> {
        let path = format!("/{}/ticker", value_in_currency_pairs(&self.pair));
        let builder = getPrivateUriBuilder(path);
        let resp = reqwest::get(builder.build().unwrap().to_string())
            .await?
            .json::<TickerResponse>()
            .await?;
        Ok(resp.into())
    }

    fn get_private_uri_builder(path: String) -> uri::Builder {
        uri::Builder::new()
            .scheme("https")
            .authority(ENDPOINT)
            .path_and_query(path)
    }

    fn make_private_request_header(&self, nonce: u128, sign: String) -> Vec<()> {}

    fn get_private_get_request_header(&self, path: String, query: String) -> Vec<()> {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let message = nonce.to_string() + &path + &query;
        self.make_private_request_header(nonce, self.create_sign(message))
    }

    fn get_private_post_request_header(&self, json: String) -> Vec<()> {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let message = nonce.to_string() + &json;
        self.make_private_request_header(nonce, self.create_sign(message))
    }

    fn create_sign(&self, message: String) -> String {
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        code_bytes.to_string()
    }
}
