#![doc = include_str!("../README.md")]
extern crate core;

use crate::error::Error;
use crate::models::{
    ClientIps, ConfigJson, CpuHistoryPoint, EchCert, LoginInfo, Mldsa65, Mlkem768, Response,
    ServerStatus, Uuid, VlessEnc, X25519Cert,
};
pub use client::Client;
pub use client::ClientOptions;
pub use client::LoginResult;
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
pub type ClientIpsResponse = Response<ClientIps>;
pub type DeleteInboundResponse = Response<u64>;
pub type OnlineClientsResponse = Response<Option<Vec<String>>>;
pub type StringResponse = Response<String>;
pub type JsonResponse = Response<Value>;
pub type OptStringVecResponse = Response<Option<Vec<String>>>;
pub type StringVecResponse = Response<Vec<String>>;
pub type CpuHistoryResponse = Response<Option<Vec<CpuHistoryPoint>>>;
pub type UuidResponse = Response<Uuid>;
pub type ServerStatusResponse = Response<Option<ServerStatus>>;
pub type ConfigJsonResponse = Response<ConfigJson>;
pub type X25519CertResponse = Response<X25519Cert>;
pub type Mldsa65Response = Response<Mldsa65>;
pub type Mlkem768Response = Response<Mlkem768>;
pub type VlessEncResponse = Response<VlessEnc>;
pub type EchCertResponse = Response<EchCert>;
pub type LoginResponse = Response<Option<LoginInfo>>;
