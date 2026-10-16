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

/// An on-demand response byte stream; errors must not expose credentials or raw response bodies.
pub type ResponseBody = Pin<Box<dyn Stream<Item = Result<Bytes, SdkError>> + Send + 'static>>;

/// A signed HTTP request; transports must not re-encode its body, alter signed fields, or replay it automatically.
#[derive(Clone)]
pub struct TransportRequest {
    /// The HTTP method; business calls use POST and credential metadata reads use GET.
    pub method: Method,
    /// The validated target URI.
    pub uri: Uri,
    /// Common parameters, authentication, and user-supplied headers.
    pub headers: HeaderMap,
    /// The exact body bytes used for signing.
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

/// A transport response whose body is streamed asynchronously instead of collected without bounds.
pub struct TransportResponse {
    /// The HTTP status code.
    pub status: StatusCode,
    /// The HTTP response headers.
    pub headers: HeaderMap,
    /// The response body; dropping it must release underlying network resources.
    pub body: ResponseBody,
}

impl TransportResponse {
    /// Creates a response from buffered bytes, useful for implementing a mock transport.
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

/// A replaceable HTTP boundary; implementations must disable redirects and automatic retries and preserve request bytes.
pub trait Transport: Send + Sync {
    /// Sends once; dropping the future or body must release resources without leaving detached business tasks.
    fn send(&self, request: TransportRequest)
    -> BoxFuture<'_, Result<TransportResponse, SdkError>>;
}
