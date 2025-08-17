use reqwest::StatusCode;
use thiserror::Error;
use crate::response_ext::JsonVerboseError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Not found (404): {0}")]
    NotFound(#[source] reqwest::Error),
    #[error("Connection error: {0}")]
    Connection(#[source] reqwest::Error),
    #[error("Invalid credentials!")]
    InvalidCred,
    #[error("Error: {0}!")]
    OtherError(String),
    #[error(transparent)]
    JsonVerbose(#[from] JsonVerboseError),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if let Some(status) = err.status() {
            if status == StatusCode::NOT_FOUND {
                return Error::NotFound(err);
            }
        }
        Error::Connection(err)
    }
}
