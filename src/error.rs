use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("failed bb by {0}")]
    Code(i64),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    ParseError(#[from] serde_json::Error),
}
