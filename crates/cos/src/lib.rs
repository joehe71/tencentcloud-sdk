//! Tencent Cloud COS XML API with explicit identities, bounded buffered I/O, and no retries.
//! Independent of API 3.0 JSON/TC3. Requires an application-owned Tokio time driver.
#![doc = include_str!("../README.md")]
mod client;
mod error;
/// XML request and response models for supported COS operations.
pub mod models;
mod protocol;
pub use client::{Client, ClientBuilder, PresignedUrl, Response};
pub use error::{Error, ServiceError};
