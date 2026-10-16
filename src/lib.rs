//! The unified Tencent Cloud SDK entry point, with no products or HTTP implementation enabled by default.
//!
//! Re-exports core types; `transport-reqwest` enables the default transport with a shared connection pool.

pub use tencentcloud_sdk_core::{
    ApiResponse, BoxFuture, Client, ClientBuilder, CommonClient, Credentials, Endpoint, Language,
    Operation, RequestOptions, ResponseMetadata, SdkError, ServiceError,
};
pub use tencentcloud_sdk_core::{credentials, runtime, transport};
