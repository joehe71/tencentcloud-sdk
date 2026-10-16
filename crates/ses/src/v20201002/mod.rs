//! SES 2020-10-02: explicit, single-attempt calls with no implicit pagination.
use serde::{Serialize, de::DeserializeOwned};
use std::fmt;
use tencentcloud_sdk_core::{
    ApiResponse, Client as CoreClient, Operation, RequestOptions, SdkError,
};

pub mod models;
mod sealed {
    pub trait Sealed {}
}

/// A sealed official request, paired with its response and original action name.
pub trait Request: Serialize + Sync + sealed::Sealed {
    /// The operation's strongly typed response.
    type Response: DeserializeOwned;
    /// The official action, including its original capitalization.
    const ACTION: &'static str;
}

/// A lightweight product view over a shared immutable core client.
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
    /// Wraps an existing core client without I/O or credential lookup.
    #[must_use]
    pub fn new(core: CoreClient) -> Self {
        Self { core }
    }

    /// Returns the underlying client for shared configuration and other product views.
    #[must_use]
    pub fn core(&self) -> &CoreClient {
        &self.core
    }

    /// Executes one typed operation, without automatic retries or pagination.
    ///
    /// # Errors
    /// Propagates the original core encoding, credential, transport, timeout or service error.
    pub async fn execute<I: Request>(
        &self,
        request: &I,
    ) -> Result<ApiResponse<I::Response>, SdkError> {
        self.execute_with_options(request, &RequestOptions::default())
            .await
    }

    /// Executes one typed operation with per-call region, language, header and timeout options.
    ///
    /// # Errors
    /// Propagates the original core error without changing its code or request identifier.
    pub async fn execute_with_options<I: Request>(
        &self,
        request: &I,
        options: &RequestOptions,
    ) -> Result<ApiResponse<I::Response>, SdkError> {
        let operation = Operation::new("ses", "2020-10-02", I::ACTION)?;
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
            /// Propagates the original core error; side effects are never automatically replayed.
            pub async fn $method(&self, request: &models::$input) -> Result<ApiResponse<models::$output>, SdkError> {
                self.execute(request).await
            }
        })+
    };
}
include!("operations.rs");
