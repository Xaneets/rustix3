extern crate core;

use crate::error::Error;
use crate::models::{CpuHistoryPoint, Response, Uuid};
pub use client::Client;
use models::{ClientStats, Inbounds};
use serde_json::Value;

pub mod client;
pub mod error;
pub mod inbounds;
pub mod models;
pub mod response_ext;

pub type Result<T> = std::result::Result<T, Error>;

pub type NullObjectResponse = Response<Option<()>>;
pub type InboundsResponse = Response<Vec<Inbounds>>;
pub type InboundResponse = Response<Inbounds>;
pub type ClientsStatsVecResponse = Response<Vec<ClientStats>>;
pub type ClientsStatsResponse = Response<ClientStats>;
pub type ClientIpsResponse = Response<String>; // todo ip struct | result [ip, ip] or No ip record string custom deserializer
pub type DeleteInboundResponse = Response<u64>;
pub type OnlineClientsResponse = Response<Option<Vec<String>>>;
pub type StringResponse = Response<String>;
pub type JsonResponse = Response<Value>;
pub type OptStringVecResponse = Response<Option<Vec<String>>>;
pub type StringVecResponse = Response<Vec<String>>;
pub type CpuHistoryResponse = Response<Vec<CpuHistoryPoint>>;
pub type UuidRespose = Response<Uuid>;
