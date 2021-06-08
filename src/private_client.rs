use crate::model::enums::*;
use crate::model::request::*;
use crate::model::response::*;
use crate::model::*;
use hmac::{Hmac, Mac, NewMac};
use http::uri;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::Deserialize;
use sha2::Sha256;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const ENDPOINT: &str = "api.bitbank.cc";

type HmacSha256 = Hmac<Sha256>;

pub struct Bitbankcc {
    key: String,
    secret: String,
}

impl Bitbankcc {
    pub fn new(key: String, secret: String) -> Bitbankcc {
        Bitbankcc { key, secret }
    }
}

impl Bitbankcc {
    #[tokio::main]
    pub async fn get_assets(&self) -> Result<Assets, Box<dyn std::error::Error>> {
        let path = "/v1/user/assets";
        let builder = self.get_private_uri_builder(path);
        let client = reqwest::Client::new();
        let headers = self.get_private_get_request_header(path, "");
        let resp = client
            .get(builder.build().unwrap().to_string())
            .headers(headers)
            .send()
            .await?
            .json::<AssetsResponse>()
            .await?;
        Ok(resp.into())
    }

    fn get_private_uri_builder(&self, path: &str) -> uri::Builder {
        uri::Builder::new()
            .scheme("https")
            .authority(ENDPOINT)
            .path_and_query(path)
    }

    fn make_private_request_header(&self, nonce: u128, sign: String) -> HeaderMap<HeaderValue> {
        let mut headers = HeaderMap::with_capacity(4);
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/json; charset=utf-8").unwrap(),
        );
        headers.insert(
            "ACCESS-KEY",
            HeaderValue::from_str(self.key.as_str()).unwrap(),
        );
        headers.insert(
            "ACCESS-NONCE",
            HeaderValue::from_str(nonce.to_string().as_str()).unwrap(),
        );
        headers.insert(
            "ACCESS-SIGNATURE",
            HeaderValue::from_str(sign.as_str()).unwrap(),
        );
        headers
    }

    fn get_private_get_request_header(&self, path: &str, query: &str) -> HeaderMap {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let message = nonce.to_string() + &path + &query;
        self.make_private_request_header(nonce, self.create_sign(message))
    }

    fn get_private_post_request_header(&self, json: String) -> HeaderMap {
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
        hex::encode(code_bytes)
    }
}
