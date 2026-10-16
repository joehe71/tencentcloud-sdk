use http::StatusCode;

/// Common response metadata without retaining an additional copy of the full response body.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ResponseMetadata {
    /// The Tencent Cloud `RequestId`, taken from JSON or falling back to the response header.
    pub request_id: Option<String>,
    /// The HTTP status code.
    pub status: StatusCode,
}

/// A strongly typed response that owns its data.
#[derive(Debug)]
#[non_exhaustive]
pub struct ApiResponse<T> {
    /// The decoded business object inside Response.
    pub data: T,
    /// Common response metadata.
    pub metadata: ResponseMetadata,
}
