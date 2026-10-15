//! 产品无关的腾讯云 API 3.0 内核。
//!
//! 提供 TC3 签名、POST JSON 通用调用以及可替换的凭证和 HTTP 传输。
//! 异步调用需要启用了时间驱动的 Tokio Runtime；本库不会创建 Runtime。
//! 默认不启用网络传输，不读取环境凭证，不自动重试。

use std::{future::Future, pin::Pin};

#[cfg(test)]
mod benchmarks;

mod client;
mod common;
/// 凭证快照与可替换的凭证来源。
pub mod credentials;
mod endpoint;
mod error;
mod operation;
mod protocol;
mod response;
/// 可替换时钟，便于固定时间的离线测试。
pub mod runtime;
mod signing;
/// HTTP 传输边界及可选的 reqwest 实现。
pub mod transport;

pub use client::{Client, ClientBuilder};
pub use common::CommonClient;
pub use credentials::Credentials;
pub use endpoint::Endpoint;
pub use error::{SdkError, ServiceError};
pub use operation::{Language, Operation, RequestOptions};
pub use response::{ApiResponse, ResponseMetadata};

/// 可跨线程的异步结果，用于可对象化的扩展接口。
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
