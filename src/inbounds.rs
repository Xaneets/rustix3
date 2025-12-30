use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum InboundProtocols {
    Vmess,
    Vless,
    Trojan,
    #[serde(rename = "shadowsocks")]
    ShadowsSocks,
    #[serde(rename = "dokodemo-door")]
    DokodemoDoor,
    Socks,
    Http,
    Wireguard,
    #[serde(other)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum TransportProtocol {
    Tcp,
    #[serde(rename = "kcp")]
    MKCP,
    #[serde(rename = "ws")]
    WebSocket,
    GRPC,
    HTTPUpgrade,
    XHTTP,
    #[serde(other)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum SSMethods {
    #[serde(rename = "aes-256-gcm")]
    AES256Gcm,
    #[serde(rename = "aes-128-gcm")]
    AES128Gcm,
    #[serde(rename = "chacha20-poly1305")]
    CHACHA20Poly1305,
    #[serde(rename = "chacha20-ietf-poly1305")]
    CHACHA20IetfPoly1305,
    #[serde(rename = "xchacha20-poly1305")]
    XCHACHA20Poly1305,
    #[serde(rename = "xchacha20-ietf-poly1305")]
    XCHACHA20IetfPoly1305,
    #[serde(rename = "2022-blake3-aes-128-gcm")]
    Blake3Aes128Gcm,
    #[serde(rename = "2022-blake3-aes-256-gcm")]
    Blake3Aes256Gcm,
    #[serde(rename = "2022-blake3-chacha20-poly1305")]
    Blake3Chacha20Poly1305,
    #[serde(other)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum TlsFlowControl {
    #[serde(rename = "xtls-rprx-vision")]
    Vision,
    #[serde(rename = "xtls-rprx-vision-udp443")]
    VisionUdp443,
    #[serde(other)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum TlsVersionOption {
    #[serde(rename = "1.0")]
    TLS10,
    #[serde(rename = "1.1")]
    TLS11,
    #[serde(rename = "1.2")]
    TLS12,
    #[serde(rename = "1.3")]
    TLS13,
    #[serde(other)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum TlsCipherOption {
    #[serde(rename = "TLS_AES_128_GCM_SHA256")]
    AES128Gcm,
    #[serde(rename = "TLS_AES_256_GCM_SHA384")]
    AES256Gcm,
    #[serde(rename = "TLS_CHACHA20_POLY1305_SHA256")]
    Chacha20Poly1305,
    #[serde(rename = "TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA")]
    EcdheEcdsaAes128Cbc,
    #[serde(rename = "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA")]
    EcdheEcdsaAes256Cbc,
    #[serde(rename = "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA")]
    EcdheRsaAes128Cbc,
    #[serde(rename = "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA")]
    EcdheRsaAes256Cbc,
    #[serde(rename = "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256")]
    EcdheEcdsaAes128Gcm,
    #[serde(rename = "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384")]
    EcdheEcdsaAes256Gcm,
    #[serde(rename = "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256")]
    EcdheRsaAes128Gcm,
    #[serde(rename = "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384")]
    EcdheRsaAes256Gcm,
    #[serde(rename = "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256")]
    EcdheEcdsaChacha20Poly1305,
    #[serde(rename = "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256")]
    EcdheRsaChacha20Poly1305,
    #[serde(other)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum UtlsFingerprint {
    #[serde(rename = "chrome")]
    UtlsChrome,
    #[serde(rename = "firefox")]
    UtlsFirefox,
    #[serde(rename = "safari")]
    UtlsSafari,
    #[serde(rename = "ios")]
    UtlsIos,
    #[serde(rename = "android")]
    UtlsAndroid,
    #[serde(rename = "edge")]
    UtlsEdge,
    #[serde(rename = "360")]
    Utls360,
    #[serde(rename = "qq")]
    UtlsQq,
    #[serde(rename = "random")]
    UtlsRandom,
    #[serde(rename = "randomized")]
    UtlsRandomized,
    #[serde(rename = "randomizednoalpn")]
    UtlsRandomizedNoAlpn,
    #[serde(rename = "unsafe")]
    UtlsUnsafe,
    #[serde(other)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum AlpnOption {
    H3,
    H2,
    #[serde(rename = "http/1.1")]
    Http1,
    #[serde(other)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum SniffingOption {
    Http,
    Tls,
    Quic,
    FakeDns,
    #[serde(other)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum UsageOption {
    Encipherment,
    Verify,
    Issue,
    #[serde(other)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum DomainStrategyOption {
    #[serde(rename = "AsIs")]
    AsIs,
    #[serde(rename = "UseIP")]
    UseIp,
    #[serde(rename = "UseIPv6v4")]
    UseIpv6v4,
    #[serde(rename = "UseIPv6")]
    UseIpv6,
    #[serde(rename = "UseIPv4v6")]
    UseIpv4v6,
    #[serde(rename = "UseIPv4")]
    UseIpv4,
    #[serde(rename = "ForceIP")]
    ForceIp,
    #[serde(rename = "ForceIPv6v4")]
    ForceIpv6v4,
    #[serde(rename = "ForceIPv6")]
    ForceIpv6,
    #[serde(rename = "ForceIPv4v6")]
    ForceIpv4v6,
    #[serde(rename = "ForceIPv4")]
    ForceIpv4,
    #[serde(other)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum TcpCongestionOption {
    Bbr,
    Cubic,
    Reno,
    #[serde(other)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum UsersSecurity {
    #[serde(rename = "aes-128-gcm")]
    Aes128Gcm,
    #[serde(rename = "chacha20-poly1305")]
    Chacha20Poly1305,
    Auto,
    None,
    Zero,
    #[serde(other)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum ModeOption {
    Auto,
    PacketUp,
    StreamUp,
    StreamOne,
    #[serde(other)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum StreamSettings {
    TlsStreamSettings,
    RealityStreamSettings,
    TcpStreamSettings,
    KcpStreamSettings,
    WsStreamSettings,
    GrpcStreamSettings,
    HttpUpgradeStreamSettings,
    xHTTPStreamSettings,
    #[serde(other)]
    Unknown,
}
