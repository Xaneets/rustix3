use crate::enums::Protocol;
use serde::{Deserialize, Deserializer, Serialize};
use std::ops::Not;

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
    #[serde(deserialize_with = "deserialize_settings")]
    pub settings: Settings,
    #[serde(rename = "streamSettings")]
    pub stream_settings: String, // todo
    pub tag: String,
    pub sniffing: String, // todo
    pub allocate: String, // todo
}

#[derive(Serialize, Deserialize)]
pub struct CreateInboundRequest {
    pub up: i64,
    pub down: i64,
    pub total: i64,
    pub remark: String,
    pub enable: bool,
    #[serde(rename = "expiryTime")]
    pub expiry_time: i64,
    pub listen: String,
    pub port: u16,
    pub protocol: Protocol,
    pub settings: Settings,
    #[serde(rename = "streamSettings")]
    pub stream_settings: String,
    pub sniffing: String,
    pub allocate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub clients: Vec<User>,
    pub decryption: String, // todo
    pub fallbacks: Vec<Fallback>,
}

fn deserialize_settings<'de, D>(deserializer: D) -> Result<Settings, D::Error>
where
    D: Deserializer<'de>,
{
    let settings_str: String = Deserialize::deserialize(deserializer)?;
    serde_json::from_str(&settings_str).map_err(serde::de::Error::custom)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub flow: String,
    pub email: String,
    pub limit_ip: u32,
    #[serde(rename = "totalGB")]
    pub total_gb: u32,
    pub expiry_time: u64,
    pub enable: bool,
    pub tg_id: String,
    pub sub_id: String,
    pub reset: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fallback {
    #[serde(rename = "SNI")]
    pub sni: String,
    #[serde(rename = "ALPN")]
    pub alpn: String,
    pub path: String,
    pub dest: String,
    pub x_ver: u16,
}
