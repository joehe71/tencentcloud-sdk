//! Secrets Manager 2019-09-23: typed calls with no implicit mutation, retries or pagination.
use serde::{Serialize, de::DeserializeOwned};
use std::fmt;
use tencentcloud_sdk_foundation::{
    ApiResponse, Client as CoreClient, Operation, RequestOptions, SdkError,
};

pub mod models;
mod sealed {
    pub trait Sealed {}
}

/// A sealed official request with its original action and strongly typed response.
pub trait Request: Serialize + Sync + sealed::Sealed {
    /// The operation's corresponding response model.
    type Response: DeserializeOwned;
    /// The original official action name.
    const ACTION: &'static str;
}

/// A cloneable product view sharing the core client, identity and transport pool.
#[derive(Clone)]
pub struct Client {
    core: CoreClient,
}
impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client").finish_non_exhaustive()
    }
}
impl Client {
    /// Wraps a configured core client without network I/O or credential lookup.
    #[must_use]
    pub fn new(core: CoreClient) -> Self {
        Self { core }
    }

    /// Returns the shared core view without exposing credential values.
    #[must_use]
    pub fn core(&self) -> &CoreClient {
        &self.core
    }

    /// Sends a typed request once, without retries, polling or automatic rotation.
    ///
    /// # Errors
    /// Returns the original core error, preserving service codes and request identifiers.
    pub async fn execute<I: Request>(
        &self,
        request: &I,
    ) -> Result<ApiResponse<I::Response>, SdkError> {
        self.execute_with_options(request, &RequestOptions::default())
            .await
    }

    /// Sends once using per-call region, language, header and timeout overrides.
    ///
    /// # Errors
    /// Returns the original core encoding, credential, transport, timeout or service error.
    pub async fn execute_with_options<I: Request>(
        &self,
        request: &I,
        options: &RequestOptions,
    ) -> Result<ApiResponse<I::Response>, SdkError> {
        let operation = Operation::new("ssm", "2019-09-23", I::ACTION)?;
        self.core
            .execute_with_options(&operation, request, options)
            .await
    }
}
macro_rules! operations {
    ($($method:ident: $input:ident => $output:ident = $action:literal;)+) => {
        $(impl sealed::Sealed for models::$input {}
        impl Request for models::$input {
            type Response = models::$output;
            const ACTION: &'static str = $action;
        }
        impl Client {
            #[doc = concat!("Executes `", $action, "` once. Use `execute_with_options` for per-call overrides.")]
            ///
            /// # Errors
            /// Propagates the original core error without replaying side effects.
            pub async fn $method(&self, request: &models::$input) -> Result<ApiResponse<models::$output>, SdkError> {
                self.execute(request).await
            }
        })+
    };
}
include!("operations.rs");
