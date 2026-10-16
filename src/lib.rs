//! The unified Tencent Cloud SDK entry point, with no products or HTTP implementation enabled by default.
//!
//! Re-exports core types; `transport-reqwest` enables the default transport with a shared connection pool.

pub use tencentcloud_sdk_foundation::{
    ApiResponse, BoxFuture, Client, ClientBuilder, CommonClient, Credentials, Endpoint, Language,
    Operation, RequestOptions, ResponseMetadata, SdkError, ServiceError,
};
pub use tencentcloud_sdk_foundation::{credentials, runtime, transport};

#[cfg(feature = "cos")]
pub use tencentcloud_sdk_cos as cos;

#[cfg(feature = "ses")]
pub use tencentcloud_sdk_ses as ses;

#[cfg(feature = "ssm")]
pub use tencentcloud_sdk_ssm as ssm;
