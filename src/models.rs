use serde::Deserialize;
use std::ops::Not;

pub mod inbounds;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    success: bool,
    #[serde(rename = "msg")]
    pub message: String,
    #[serde(rename = "obj")]
    pub object: T,
}

impl<T> Response<T> {
    pub fn is_ok(&self) -> bool {
        self.success
    }

    pub fn is_err(&self) -> bool {
        self.success.not()
    }
}

pub type NullObjectResponse = Response<Option<()>>;
pub type LoginResponse = NullObjectResponse;
