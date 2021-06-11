use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ErrorData {
    pub code: i32,
}
