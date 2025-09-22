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

- ✅ login
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
- ❌ Import inbounds
- ✅ Last online
- ❌ del Client By Email
- ✅ Server status
- ✅ Server get DB
- ✅ get Xray Version
- ✅ get Config Json
- ✅ cpu History
- ✅ get New UUID
- ✅ get New X25519 Cert
- ✅ get New mldsa65
- ✅ get New mlkem768
- ✅ get New Vless Enc
- ✅ stop Xray Service
- ✅ restart Xray Service
- ✅ install Xray version
- ✅ update Geofile
- ❌ updateGeofile/{fileName}
- ❌ logs/{count}
- ❌ xraylogs/{count}
- ✅ importDB
- ❌ get New Ech Cert


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