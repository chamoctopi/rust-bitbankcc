use crate::model::enums::*;
use crate::model::response::*;
use crate::model::*;
use crate::Error;
use hmac::{Hmac, Mac, NewMac};
use http::uri;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderValue};
use sha2::Sha256;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};

const ENDPOINT_PUBLIC: &str = "public.bitbank.cc";
const ENDPOINT_PRIVATE: &str = "api.bitbank.cc";

type HmacSha256 = Hmac<Sha256>;

#[derive(Default)]
pub struct Bitbankcc {
    key: String,
    secret: String,
}

impl Bitbankcc {
    pub fn new() -> Bitbankcc {
        Bitbankcc {
            ..Default::default()
        }
    }
    pub fn with_credentials(key: String, secret: String) -> Bitbankcc {
        Bitbankcc { key, secret }
    }
}

impl Bitbankcc {
    fn get_public_uri_builder(&self, path: String) -> uri::Builder {
        uri::Builder::new()
            .scheme("https")
            .authority(ENDPOINT_PUBLIC)
            .path_and_query(path)
    }

    fn get_private_uri_builder(&self, path: String) -> uri::Builder {
        uri::Builder::new()
            .scheme("https")
            .authority(ENDPOINT_PRIVATE)
            .path_and_query(path)
    }

    fn get_public_request_header(&self) -> HeaderMap<HeaderValue> {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/json; charset=utf-8").unwrap(),
        );
        headers
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

    fn get_private_get_request_header(
        &self,
        path: String,
        query: String,
    ) -> HeaderMap<HeaderValue> {
        let nonce = current_time_millis();
        // FIXME: encode query
        let message = nonce.to_string() + &path + &query;
        self.make_private_request_header(nonce, self.create_sign(message))
    }

    fn get_private_post_request_header(&self, json: String) -> HeaderMap<HeaderValue> {
        let nonce = current_time_millis();
        let message = nonce.to_string() + &json;
        self.make_private_request_header(nonce, self.create_sign(message))
    }

    fn create_sign(&self, message: String) -> String {
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes()).unwrap();
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
    }

    // TODO: httpExecute

    // TODO
    // async fn doHttpGet<R, D>(builder: uri::Builder) -> Result<D, MyError>
    // where
    //     R: Deserialize,
    //     D: Into<D>,
    // {
    //     let uri = builder.build().unwrap();
    //     let resp = reqwest::get(uri.to_string()).await?.json::<R>().await?;
    //     Ok(resp.into())
    // }

    // TODO: doHttpPost

    #[tokio::main]
    async fn get_public_response(&self, path: String) -> Result<Response, Error> {
        let builder = self.get_public_uri_builder(path);
        let client = reqwest::Client::new();
        let headers = self.get_public_request_header();
        Ok(client
            .get(builder.build().unwrap().to_string())
            .headers(headers)
            .send()
            .await?
            .json::<Response>()
            .await?)
    }

    #[tokio::main]
    async fn get_private_response(&self, path: String) -> Result<Response, Error> {
        let builder = self.get_private_uri_builder(path);
        let client = reqwest::Client::new();
        let headers =
            self.get_private_get_request_header("/v1/user/assets".to_string(), "".to_string());
        Ok(client
            .get(builder.build().unwrap().to_string())
            .headers(headers)
            .send()
            .await?
            .json::<Response>()
            .await?)
    }

    /*
        Public API
    */

    pub fn get_ticker(&self, pair: CurrencyPair) -> Result<Ticker, Error> {
        let path = format!("/{}/ticker", pair);
        let resp = self.get_public_response(path)?;
        Ok(TickerData::try_from(resp)?.into())
    }

    pub fn get_depth(&self, pair: CurrencyPair) -> Result<Depth, Error> {
        let path = format!("/{}/depth", pair);
        let resp = self.get_public_response(path)?;
        Ok(DepthData::try_from(resp)?.into())
    }

    pub fn get_transaction(&self, pair: CurrencyPair, yyyymmdd: &str) -> Result<(), Error> {
        todo!()
    }

    pub fn get_candlestick(
        &self,
        pair: CurrencyPair,
        r#type: CandleType,
        yyyymmdd: &str,
    ) -> Result<Candlestick, Error> {
        let path = format!("/{}/candlestick/{}/{}", pair, &r#type, yyyymmdd);
        let resp = self.get_public_response(path)?;
        Ok(CandlestickData::try_from(resp)?.into())
    }

    /*
        Private API
    */

    pub fn get_assets(&self) -> Result<Assets, Error> {
        let path = String::from("/v1/user/assets");
        let resp = self.get_private_response(path)?;
        Ok(AssetsData::try_from(resp)?.into())
    }

    pub fn get_order(&self, pair: CurrencyPair, order_id: u64) -> Result<(), Error> {
        todo!()
    }

    pub fn get_orders(&self, pair: CurrencyPair, order_ids: Vec<u64>) -> Result<(), Error> {
        todo!()
    }

    pub fn send_order(
        &self,
        pair: CurrencyPair,
        price: f64,
        amount: f64,
        side: OrderSide,
        r#type: OrderType,
    ) -> Result<(), Error> {
        todo!()
    }

    pub fn cancel_order(&self, pair: CurrencyPair, order_id: u64) -> Result<(), Error> {
        todo!()
    }

    pub fn cancel_orders(&self, pair: CurrencyPair, order_ids: Vec<u64>) -> Result<(), Error> {
        todo!()
    }

    pub fn get_active_orders(
        &self,
        pair: CurrencyPair,
        option: HashMap<String, u64>,
    ) -> Result<(), Error> {
        todo!()
    }

    pub fn get_withdrawal_accounts(&self, asset: String) -> Result<(), Error> {
        todo!()
    }

    pub fn request_withdraw(
        &self,
        asset: String,
        uuid: String,
        amount: f64,
        otp_token: String,
        sms_token: String,
    ) -> Result<(), Error> {
        todo!()
    }
}

fn current_time_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
