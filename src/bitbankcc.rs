use crate::model::enums::*;
use crate::model::request::{CancelBody, CancelsBody, OrderBody};
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
use url::Url;

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
    pub fn with_credentials(api_key: String, api_secret: String) -> Bitbankcc {
        Bitbankcc {
            key: api_key,
            secret: api_secret,
        }
    }
}

impl Bitbankcc {
    fn get_public_uri_builder(&self, path: &str) -> uri::Builder {
        uri::Builder::new()
            .scheme("https")
            .authority(ENDPOINT_PUBLIC)
            .path_and_query(path)
    }

    fn get_private_uri_builder(&self, path: &str) -> uri::Builder {
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

    fn get_private_get_request_header(&self, path: &str, query: &str) -> HeaderMap<HeaderValue> {
        let nonce = current_time_millis();
        let query_str = if query.len() > 0 {
            String::from("?") + query
        } else {
            String::from("")
        };
        let message = nonce.to_string() + path + &query_str;
        self.make_private_request_header(nonce, self.create_sign(message))
    }

    fn get_private_post_request_header(&self, json: &str) -> HeaderMap<HeaderValue> {
        let nonce = current_time_millis();
        let message = nonce.to_string() + json;
        self.make_private_request_header(nonce, self.create_sign(message))
    }

    fn create_sign(&self, message: String) -> String {
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes()).unwrap();
        mac.update(message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
    }

    #[tokio::main]
    async fn http_execute(
        &self,
        client: reqwest::Client,
        http: reqwest::RequestBuilder,
    ) -> Result<Response, Error> {
        let request = http.build().unwrap();
        Ok(client.execute(request).await?.json::<Response>().await?)
    }

    fn do_http_get(&self, url: Url, headers: HeaderMap<HeaderValue>) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let http = client.request(http::Method::GET, url).headers(headers);
        self.http_execute(client, http)
    }

    fn do_http_post(
        &self,
        url: Url,
        headers: HeaderMap<HeaderValue>,
        body: String,
    ) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let http = client
            .request(http::Method::POST, url)
            .headers(headers)
            .body(body);
        self.http_execute(client, http)
    }

    /*
        Public API
    */

    pub fn get_ticker(&self, pair: CurrencyPair) -> Result<Ticker, Error> {
        let path = format!("/{}/ticker", pair);
        let uri = self.get_public_uri_builder(&path).build()?;
        let url = Url::parse(&uri.to_string())?;
        let headers = self.get_public_request_header();
        let resp = self.do_http_get(url, headers)?;
        Ok(TickerData::try_from(resp)?.into())
    }

    pub fn get_depth(&self, pair: CurrencyPair) -> Result<Depth, Error> {
        let path = format!("/{}/depth", pair);
        let uri = self.get_public_uri_builder(&path).build()?;
        let url = Url::parse(&uri.to_string())?;
        let headers = self.get_public_request_header();
        let resp = self.do_http_get(url, headers)?;
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
        let uri = self.get_public_uri_builder(&path).build()?;
        let url = Url::parse(&uri.to_string())?;
        let headers = self.get_public_request_header();
        let resp = self.do_http_get(url, headers)?;
        Ok(CandlestickData::try_from(resp)?.into())
    }

    /*
        Private API
    */

    pub fn get_assets(&self) -> Result<Assets, Error> {
        let path = "/v1/user/assets";
        let uri = self.get_private_uri_builder(path).build()?;
        let url = Url::parse(&uri.to_string())?;
        let headers = self.get_private_get_request_header(path, "");
        let resp = self.do_http_get(url, headers)?;
        Ok(AssetsData::try_from(resp)?.into())
    }

    pub fn get_order(&self, pair: CurrencyPair, order_id: u64) -> Result<Order, Error> {
        let path = "/v1/user/spot/order";
        let uri = self.get_private_uri_builder(path).build()?;
        let url = Url::parse_with_params(
            &uri.to_string(),
            &[
                ("pair", pair.to_string()),
                ("order_id", order_id.to_string()),
            ],
        )?;
        let headers = self.get_private_get_request_header(path, url.query().unwrap());
        let resp = self.do_http_get(url, headers)?;
        Ok(OrderData::try_from(resp)?.into())
    }

    pub fn get_orders(&self, pair: CurrencyPair, order_ids: Vec<u64>) -> Result<Orders, Error> {
        let path = "/v1/user/spot/orders_info";
        let uri = self.get_private_uri_builder(path).build()?;
        let url = Url::parse(&uri.to_string())?;
        let json = CancelsBody::new(pair, order_ids).to_string();
        let headers = self.get_private_post_request_header(&json);
        let resp = self.do_http_post(url, headers, json)?;
        Ok(OrdersData::try_from(resp)?.into())
    }

    pub fn send_order(
        &self,
        pair: CurrencyPair,
        price: f64,
        amount: f64,
        side: OrderSide,
        r#type: OrderType,
        post_only: bool,
    ) -> Result<Order, Error> {
        let path = "/v1/user/spot/order";
        let uri = self.get_private_uri_builder(path).build()?;
        let url = Url::parse(&uri.to_string())?;
        let json = OrderBody::new(pair, price, amount, side, r#type, post_only).to_string();
        let headers = self.get_private_post_request_header(&json);
        let resp = self.do_http_post(url, headers, json)?;
        Ok(OrderData::try_from(resp)?.into())
    }

    pub fn cancel_order(&self, pair: CurrencyPair, order_id: u64) -> Result<Order, Error> {
        let path = "/v1/user/spot/cancel_order";
        let uri = self.get_private_uri_builder(path).build()?;
        let url = Url::parse(&uri.to_string())?;
        let json = CancelBody::new(pair, order_id).to_string();
        let headers = self.get_private_post_request_header(&json);
        let resp = self.do_http_post(url, headers, json)?;
        Ok(OrderData::try_from(resp)?.into())
    }

    pub fn cancel_orders(&self, pair: CurrencyPair, order_ids: Vec<u64>) -> Result<Orders, Error> {
        let path = "/v1/user/spot/cancel_orders";
        let uri = self.get_private_uri_builder(path).build()?;
        let url = Url::parse(&uri.to_string())?;
        let json = CancelsBody::new(pair, order_ids).to_string();
        let headers = self.get_private_post_request_header(&json);
        let resp = self.do_http_post(url, headers, json)?;
        Ok(OrdersData::try_from(resp)?.into())
    }

    pub fn get_active_orders(
        &self,
        pair: CurrencyPair,
        option: HashMap<String, u64>,
    ) -> Result<Orders, Error> {
        let path = "/v1/user/spot/active_orders";
        let uri = self.get_private_uri_builder(path).build()?;
        let mut params = HashMap::<String, String>::new();
        params.insert("pair".to_string(), pair.to_string());
        for (k, v) in option {
            params.insert(k, v.to_string());
        }
        let url = Url::parse_with_params(&uri.to_string(), params)?;
        let headers = self.get_private_get_request_header(path, url.query().unwrap());
        let resp = self.do_http_get(url, headers)?;
        Ok(OrdersData::try_from(resp)?.into())
    }

    pub fn get_withdrawal_accounts(&self, asset: String) -> Result<(), Error> {
        todo!()
    }

    pub fn request_withdraw(
        &self,
        _asset: String,
        _uuid: String,
        _amount: f64,
        _otp_token: String,
        _sms_token: String,
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
