#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed bb by {0}")]
    Code(i64),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    ParseError(#[from] serde_json::Error),
    #[error(transparent)]
    HttpError(#[from] http::Error),
}
