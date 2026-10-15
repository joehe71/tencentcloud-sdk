use std::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_core::Stream;
use http::{HeaderMap, Method, StatusCode, Uri};

use crate::{BoxFuture, SdkError};

#[cfg(feature = "transport-reqwest")]
mod reqwest;
#[cfg(feature = "transport-reqwest")]
pub use reqwest::ReqwestTransport;

/// 按需读取的响应字节流；错误不得包含凭证或原始响应体。
pub type ResponseBody = Pin<Box<dyn Stream<Item = Result<Bytes, SdkError>> + Send + 'static>>;

/// 已签名的 HTTP 请求。传输实现不得重编码 body、修改签名字段或自动重放。
#[derive(Clone)]
pub struct TransportRequest {
    /// HTTP 方法；当前核心仅产生 POST。
    pub method: Method,
    /// 已验证的目标 URI。
    pub uri: Uri,
    /// 公共参数、认证与用户 Header。
    pub headers: HeaderMap,
    /// 与签名一致的精确字节体。
    pub body: Bytes,
}

impl fmt::Debug for TransportRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransportRequest")
            .field("method", &self.method)
            .field("uri", &self.uri)
            .field("body_len", &self.body.len())
            .finish_non_exhaustive()
    }
}

/// 传输响应，响应体通过异步 Stream 读取而非无界聚合。
pub struct TransportResponse {
    /// HTTP 状态码。
    pub status: StatusCode,
    /// HTTP 响应 Header。
    pub headers: HeaderMap,
    /// 响应体；丢弃时应释放底层网络资源。
    pub body: ResponseBody,
}

impl TransportResponse {
    /// 由已经缓冲的字节构造响应，方便实现 `MockTransport`。
    #[must_use]
    pub fn from_bytes(status: StatusCode, headers: HeaderMap, body: impl Into<Bytes>) -> Self {
        Self {
            status,
            headers,
            body: Box::pin(OneChunk(Some(body.into()))),
        }
    }
}

struct OneChunk(Option<Bytes>);

impl Stream for OneChunk {
    type Item = Result<Bytes, SdkError>;
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.0.take().map(Ok))
    }
}

/// 可替换 HTTP 边界。自定义实现负责禁止重定向与自动重试，遵守请求字节不变约束。
pub trait Transport: Send + Sync {
    /// 发送一次请求；future 或 body 被丢弃时释放资源，不遗留 detached 业务任务。
    fn send(&self, request: TransportRequest)
    -> BoxFuture<'_, Result<TransportResponse, SdkError>>;
}
