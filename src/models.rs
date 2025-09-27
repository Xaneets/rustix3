use crate::inbounds::InboundProtocols;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{json::JsonString, serde_as};
use std::ops::Not;

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    pub protocol: InboundProtocols,
    #[serde(
        deserialize_with = "de_settings_from_str_or_map",
        serialize_with   = "se_settings_as_str"
    )]
    pub settings: Settings,
    #[serde(rename = "streamSettings")]
    pub stream_settings: String, // todo
    pub tag: String,
    pub sniffing: String, // todo
    pub allocate: Option<String>, // todo
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
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
    pub protocol: InboundProtocols,
    #[serde_as(as = "JsonString<_>")]
    pub settings: Settings,
    #[serde(rename = "streamSettings")]
    pub stream_settings: String,
    pub sniffing: String,
    pub allocate: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub clients: Vec<User>,
    pub decryption: String, // todo
    pub fallbacks: Vec<Fallback>,
}

fn de_settings_from_str_or_map<'de, D>(d: D) -> Result<Settings, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Wire {
        Str(String),
        Map(Settings),
    }
    match Wire::deserialize(d)? {
        Wire::Str(s) => serde_json::from_str(&s).map_err(serde::de::Error::custom),
        Wire::Map(m) => Ok(m),
    }
}

fn se_settings_as_str<S>(value: &Settings, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let json = serde_json::to_string(value).map_err(serde::ser::Error::custom)?;
    s.serialize_str(&json)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub tg_id: TgId,
    pub sub_id: String,
    pub reset: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TgId {
    String(String),
    Int(u32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientSettings {
    pub clients: Vec<User>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientRequest {
    pub id: u64,
    #[serde_as(as = "JsonString<_>")]
    pub settings: ClientSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuHistoryPoint {
    pub cpu: f64,
    pub t: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Uuid {
    pub uuid: String
}