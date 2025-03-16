extern crate core;
use crate::error::Error;
use crate::models::Response;
pub use client::Client;
use models::{ClientStats, Inbounds};

pub mod client;
pub mod inbounds;
pub mod error;
pub mod models;

pub type Result<T> = std::result::Result<T, Error>;

pub type NullObjectResponse = Response<Option<()>>;
pub type InboundsResponse = Response<Vec<Inbounds>>;
pub type InboundResponse = Response<Inbounds>;
pub type ClientsStatsVecResponse = Response<Vec<ClientStats>>;
pub type ClientsStatsResponse = Response<ClientStats>;
pub type ClientIpsResponse = Response<String>; // todo ip struct | result [ip, ip] or No ip record string custom deserializer
pub type DeleteInboundResponse = Response<u64>;
pub type OnlineClientsResponse = Response<Vec<String>>;
