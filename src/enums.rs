use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Vmess,
    Vless,
    Trojan,
    ShadowsSocks,
    #[serde(rename = "dokodemo-door")]
    DokodemoDoor,
    Socks,
    Http,
    Wireguard,
}
