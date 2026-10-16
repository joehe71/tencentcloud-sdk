use http::StatusCode;
use std::{error::Error as StdError, fmt};
use tencentcloud_sdk_core::SdkError;

/// A COS service error preserving official fields while redacting the message from formatting.
#[derive(Clone)]
#[non_exhaustive]
pub struct ServiceError {
    /// Official COS error code.
    pub code: String,
    /// Official message; access explicitly and avoid logging sensitive object details.
    pub message: String,
    /// COS request identifier, including header fallback.
    pub request_id: Option<String>,
    /// HTTP status, which can be 200 for an embedded completion error.
    pub status: StatusCode,
}
impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "COS service error: {} (HTTP {})", self.code, self.status)
    }
}
impl fmt::Debug for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
impl StdError for ServiceError {}

/// A shared SDK error or an official COS XML service error.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Configuration, credentials, transport, protocol, bounds or deadline failure.
    Sdk(SdkError),
    /// COS service failure, without a remapped code.
    Service(ServiceError),
}
impl From<SdkError> for Error {
    fn from(error: SdkError) -> Self {
        Self::Sdk(error)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sdk(error) => fmt::Display::fmt(error, f),
            Self::Service(error) => fmt::Display::fmt(error, f),
        }
    }
}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(match self {
            Self::Sdk(error) => error,
            Self::Service(error) => error,
        })
    }
}
