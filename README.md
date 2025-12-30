[![Build](https://github.com/Xaneets/rustix3/actions/workflows/rust.yml/badge.svg)](https://github.com/Xaneets/rustix3/actions/workflows/rust.yml)
[![e2e](https://github.com/Xaneets/rustix3/actions/workflows/e2e.yml/badge.svg)](https://github.com/Xaneets/rustix3/actions/workflows/e2e.yml)
[![Crates.io](https://img.shields.io/crates/v/rustix3.svg)](https://crates.io/crates/rustix3)

# rustix3

Unofficial Rust client for the **3x-ui** panel API (Xray-core).  
Provides typed models and high-level methods for common panel operations.

> Note: Some 3x-ui endpoints expect certain nested structures to be sent as **JSON strings** (e.g., inbound `settings`).
> The client models handle these specifics transparently.

---

## Implemented endpoints

- ✅ Login
- ✅ Inbounds
- ✅ Inbound
- ✅ Client traffics with email
- ✅ Client traffics with id
- ✅ TG Send backup to admin
- ✅ Client IP address
- ✅ Add inbound
- ✅ Add client to inbound
- ✅ Update inbound
- ✅ Update client
- ✅ Clear client IP address
- ✅ Reset traffics of all inbound
- ✅ Reset traffics of all clients in an inbound
- ✅ Reset client traffics
- ✅ Delete client
- ✅ Delete inbound
- ✅ Delete depleted clients
- ✅ Online clients
- ✅ Import inbounds
- ✅ Last online
- ✅ Del Client By Email
- ✅ Server status
- ✅ Server get DB
- ✅ Get Xray Version
- ✅ Get Config Json
- ✅ Cpu History
- ✅ Get New UUID
- ✅ Get New X25519 Cert
- ✅ Get New mldsa65
- ✅ Get New mlkem768
- ✅ Get New Vless Enc
- ✅ Stop Xray Service
- ✅ Restart Xray Service
- ✅ Install Xray version
- ✅ Update Geofile
- ✅ Update Geofile/{fileName}
- ✅ Logs
- ✅ Xraylogs
- ✅ ImportDB
- ✅ Get New Ech Cert


---

## Installation

Use the Git dependency directly:

```toml
[dependencies]
rustix3 = { git = "https://github.com/Xaneets/rustix3", branch = "main" }
```

---

## Quick start

```rust
use rustix3::client::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let base_url = "http://127.0.0.1:2053/";
    let username = "admin";
    let password = "admin";

    let client = Client::new(username, password, base_url).await?;

    // Example: list inbounds
    let inbounds = client.get_inbounds_list().await?;
    println!("{:#?}", inbounds);

    Ok(())
}

```

## Configure retry and timeouts

```rust
use rustix3::{Client, ClientOptions};
use reqwest::Method;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut options = ClientOptions::default();
    options.retry_count = 3;
    options.retry_base_delay = Duration::from_millis(300);
    options.retry_max_delay = Duration::from_secs(3);
    options.retry_methods = vec![Method::GET, Method::HEAD];
    options.connect_timeout = Duration::from_secs(5);
    options.request_timeout = Duration::from_secs(20);

    let client = Client::new_with_options("admin", "admin", "http://127.0.0.1:2053/", options).await?;
    let _ = client.get_inbounds_list().await?;
    Ok(())
}
```

## Error handling

All API responses use a `success/msg/obj` envelope. When `success=false`, the client returns
`Error::ApiError { message }` with the server-provided `msg`.

Network and protocol errors are mapped to:
- `Error::InvalidUrl` for malformed base URL
- `Error::NotFound` for HTTP 404
- `Error::Connection` for other reqwest failures
- `Error::JsonVerbose` for JSON decoding errors (includes JSON path)

Example:

```rust
use rustix3::Error;

match client.get_inbounds_list().await {
    Ok(inbounds) => println!("count={}", inbounds.len()),
    Err(Error::ApiError { message }) => eprintln!("api error: {}", message),
    Err(e) => eprintln!("request error: {}", e),
}
```

---

## Create inbound example

```rust
use rustix3::client::Client;
use rustix3::inbounds::InboundProtocols;
use rustix3::inbounds::TransportProtocol;
use rustix3::models::{CreateInboundRequest, SettingsRequest, Fallback, Sniffing, StreamSettings, TcpHeader, TcpSettings};
use serde_json::json;

fn default_stream_settings() -> StreamSettings {
    StreamSettings {
        network: Some(TransportProtocol::Tcp),
        security: Some("none".into()),
        external_proxy: Some(Vec::new()),
        tcp_settings: Some(TcpSettings {
            accept_proxy_protocol: Some(false),
            header: Some(TcpHeader {
                header_type: Some("none".into()),
                extra: Default::default(),
            }),
            extra: Default::default(),
        }),
        ws_settings: None,
        grpc_settings: None,
        kcp_settings: None,
        http_upgrade_settings: None,
        xhttp_settings: None,
        extra: Default::default(),
    }
}

fn default_sniffing() -> Sniffing {
    Sniffing {
        enabled: false,
        dest_override: vec![
            rustix3::inbounds::SniffingOption::Http,
            rustix3::inbounds::SniffingOption::Tls,
            rustix3::inbounds::SniffingOption::Quic,
            rustix3::inbounds::SniffingOption::FakeDns,
        ],
        metadata_only: false,
        route_only: false,
        extra: Default::default(),
    }
}

fn default_allocate() -> serde_json::Value {
    json!({
        "strategy": "always",
        "refresh": 5,
        "concurrency": 3
    })
}

async fn create_inbound(client: &Client) -> anyhow::Result<()> {
    let req = CreateInboundRequest {
        up: 0,
        down: 0,
        total: 0,
        remark: "example-inbound".into(),
        enable: true,
        expiry_time: 0,
        listen: "0.0.0.0".into(),
        port: 31001,
        protocol: InboundProtocols::Vless,
        settings: SettingsRequest {
            clients: vec![],
            decryption: Some("none".into()),
            encryption: Some("none".into()),
            fallbacks: Vec::<Fallback>::new(),
        },
        stream_settings: default_stream_settings(),
        sniffing: default_sniffing(),
        allocate: default_allocate(),
    };

    let _created = client.add_inbound(&req).await?;
    Ok(())
}
```

## Add client example

```rust
use rustix3::client::Client;
use rustix3::models::{ClientRequest, ClientSettings, UserRequest, TgId};
use uuid::Uuid;

async fn add_client(client: &Client, inbound_id: u64) -> anyhow::Result<()> {
    let user_id = Uuid::new_v4().to_string();
    let email = format!("{user_id}@example.com");
    let sub_id = Uuid::new_v4().simple().to_string();

    let user = UserRequest {
        id: user_id,
        flow: String::new(),
        email,
        limit_ip: 2,
        total_gb: 100,
        expiry_time: 0,
        enable: true,
        tg_id: Some(TgId::Int(0)),
        sub_id,
        reset: 0,
    };

    let req = ClientRequest {
        id: inbound_id,
        settings: ClientSettings { clients: vec![user] },
    };

    client.add_client_to_inbound(&req).await?;
    Ok(())
}
```
