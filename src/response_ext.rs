use log::trace;
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsonVerboseError {
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
    #[error("http status {status}: {body}")]
    Status { status: StatusCode, body: String },
    #[error("json decode at {path}: {source}. body={body}")]
    Decode {
        source: serde_json::Error,
        path: String,
        body: String,
    },
}

pub type Result<T> = std::result::Result<T, JsonVerboseError>;

pub trait ResponseJsonVerboseExt {
    fn json_verbose<T: DeserializeOwned + 'static>(
        self,
    ) -> futures::future::BoxFuture<'static, Result<T>>;
}

impl ResponseJsonVerboseExt for Response {
    fn json_verbose<T: DeserializeOwned + 'static>(
        self,
    ) -> futures::future::BoxFuture<'static, Result<T>> {
        Box::pin(async move {
            let status = self.status();
            let bytes = self.bytes().await?;
            let body = String::from_utf8(bytes.to_vec())?;
            if !status.is_success() {
                trace!("status={} body={}", status.as_u16(), body);
                return Err(JsonVerboseError::Status { status, body });
            }
            match serde_json::from_str::<Value>(&body) {
                Ok(v) => trace!("json={}", v),
                Err(_) => trace!("raw={}", body),
            }
            let mut de = serde_json::Deserializer::from_str(&body);
            match serde_path_to_error::deserialize::<_, T>(&mut de) {
                Ok(val) => Ok(val),
                Err(e) => {
                    let path = e.path().to_string();
                    Err(JsonVerboseError::Decode {
                        source: e.into_inner(),
                        path,
                        body,
                    })
                }
            }
        })
    }
}
