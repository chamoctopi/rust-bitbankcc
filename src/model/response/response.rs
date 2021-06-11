use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Response {
    pub success: u8,
    pub data: Value,
}
