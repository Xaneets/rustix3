extern crate core;

use crate::error::Error;

pub mod client;
pub mod error;
pub mod models;
pub mod enums;

pub use client::Client;

pub type Result<T> = std::result::Result<T, Error>;
