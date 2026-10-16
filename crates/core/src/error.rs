use std::{error::Error, fmt};

use http::StatusCode;

/// A Tencent Cloud service error whose message is accessible explicitly but omitted from default formatting.
#[derive(Clone)]
#[non_exhaustive]
pub struct ServiceError {
    /// The official error code, without remapping.
    pub code: String,
    /// The official error message, which may contain business data.
    pub message: String,
    /// The Tencent Cloud request identifier.
    pub request_id: Option<String>,
    /// The HTTP status code.
    pub status: StatusCode,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tencent Cloud service error: {} (HTTP {})",
            self.code, self.status
        )
    }
}

impl fmt::Debug for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for ServiceError {}

/// An SDK error preserving its category and source without exposing response bodies or credentials by default.
#[non_exhaustive]
pub enum SdkError {
    /// Invalid client configuration.
    Configuration(&'static str),
    /// Missing or invalid credentials.
    Credentials(&'static str),
    /// Invalid signing input or a signing computation failure.
    Signing(&'static str),
    /// Failure to encode a request as JSON.
    Serialization(serde_json::Error),
    /// An underlying transport error, accessible explicitly through `Error::source`.
    Transport(Box<dyn Error + Send + Sync>),
    /// The call deadline or an underlying connection timeout was exceeded.
    Timeout,
    /// A request or response body exceeded the configured limit.
    BodyTooLarge {
        /// The maximum allowed size in bytes.
        limit: usize,
    },
    /// A response that does not follow the Tencent Cloud JSON protocol.
    Protocol {
        /// A description that does not include the raw response.
        message: &'static str,
        /// The received HTTP status code.
        status: StatusCode,
        /// The request identifier extracted from the response.
        request_id: Option<String>,
    },
    /// A Tencent Cloud service error.
    Service(ServiceError),
}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Configuration(message) => write!(f, "client configuration error: {message}"),
            Self::Credentials(message) => write!(f, "credentials error: {message}"),
            Self::Signing(message) => write!(f, "signing error: {message}"),
            Self::Serialization(_) => f.write_str("failed to encode request JSON"),
            Self::Transport(_) => f.write_str("HTTP transport failed"),
            Self::Timeout => f.write_str("call timed out"),
            Self::BodyTooLarge { limit } => {
                write!(f, "request or response body exceeds {limit} bytes")
            }
            Self::Protocol {
                message, status, ..
            } => {
                write!(f, "response protocol error: {message} (HTTP {status})")
            }
            Self::Service(error) => fmt::Display::fmt(error, f),
        }
    }
}

impl fmt::Debug for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for SdkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Serialization(error) => Some(error),
            Self::Transport(error) => Some(error.as_ref()),
            Self::Service(error) => Some(error),
            _ => None,
        }
    }
}
