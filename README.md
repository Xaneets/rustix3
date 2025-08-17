[![Build](https://github.com/Xaneets/rustix3/actions/workflows/rust.yml/badge.svg)](https://github.com/Xaneets/rustix3/actions/workflows/rust.yml)
[![e2e](https://github.com/Xaneets/rustix3/actions/workflows/e2e.yml/badge.svg)](https://github.com/Xaneets/rustix3/actions/workflows/e2e.yml)
[![Crates.io](https://img.shields.io/crates/v/rustix3.svg)](https://crates.io/crates/rustix3)

# rustix3

Unofficial Rust client for the **3x-ui** panel API (Xray-core).  
Provides typed models and high-level methods for common panel operations.

> Note: Some 3x-ui endpoints expect certain nested structures to be sent as **JSON strings** (e.g., inbound `settings`). The client models handle these specifics transparently.

---

## Implemented endpoints

- [x] login
- [x] Inbounds
- [x] Inbound
- [x] Client traffics with email
- [x] Client traffics with id
- [x] TG Send backup to admin
- [x] Client IP address
- [x] Add inbound
- [x] Add client to inbound
- [x] Update inbound
- [x] Update client
- [x] Clear client IP address
- [x] Reset traffics of all inbound
- [x] Reset traffics of all clients in an inbound
- [x] Reset client traffics
- [x] Delete client
- [x] Delete inbound
- [x] Delete depleted clients
- [x] Online clients

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