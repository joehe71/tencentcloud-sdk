//! A product-independent core for Tencent Cloud API 3.0.
//!
//! Provides TC3 signing, generic POST JSON calls, and replaceable credentials and HTTP transports.
//! Async calls require a Tokio runtime with the time driver enabled; this library does not create one.
//! Does not enable network transport, read environment credentials, or retry automatically by default.

use std::{future::Future, pin::Pin};

#[cfg(test)]
mod benchmarks;

mod client;
mod common;
/// COS XML API authentication without exposing credential secret material.
#[cfg(feature = "cos-signing")]
pub mod cos_signing;
/// Credentials snapshots and replaceable credentials providers.
pub mod credentials;
mod endpoint;
mod error;
mod operation;
mod protocol;
mod response;
/// Replaceable clocks for deterministic offline tests.
pub mod runtime;
mod signing;
/// The HTTP transport boundary and optional reqwest implementation.
pub mod transport;

pub use client::{Client, ClientBuilder};
pub use common::CommonClient;
pub use credentials::Credentials;
pub use endpoint::Endpoint;
pub use error::{SdkError, ServiceError};
pub use operation::{Language, Operation, RequestOptions};
pub use response::{ApiResponse, ResponseMetadata};

/// A sendable boxed future for object-safe extension interfaces.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
