use crate::models::Response;
use serde::{Deserialize};
use crate::enums::Protocol;

pub type InboundsResponse = Response<Vec<Inbounds>>;
pub type InboundResponse = Response<Inbounds>;
pub type ClientsStatsResponse = Response<Vec<ClientStats>>;

#[derive(Debug, Deserialize)]
pub struct ClientStats {
    pub id: u64,
    #[serde(rename = "inboundId")]
    pub inbound_id: u64,
    pub enable: bool,
    pub email: String,
    pub up: u128,
    pub down: u128,
    #[serde(rename = "expiryTime")]
    pub expiry_time: i64, // todo
    pub total: u128,
    pub reset: i64,
}

#[derive(Debug, Deserialize)]
pub struct Inbounds {
    pub id: u64,
    pub up: u128,
    pub down: u128,
    pub total: u128,
    pub remark: String,
    pub enable: bool,
    #[serde(rename = "expiryTime")]
    pub expiry_time: i64,
    #[serde(rename = "clientStats")]
    pub client_stats: Option<Vec<ClientStats>>,
    pub listen: String,
    pub port: u16,
    pub protocol: Protocol,
    pub settings: String,
    #[serde(rename = "streamSettings")]
    pub stream_settings: String, // todo
    pub tag: String,
    pub sniffing: String, // todo
    pub allocate: String, // todo
}
