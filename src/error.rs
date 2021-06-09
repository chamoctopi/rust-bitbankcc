use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("failed bb by {0}")]
    Code(i32),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
}
