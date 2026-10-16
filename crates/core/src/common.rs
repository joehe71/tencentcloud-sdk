use std::borrow::Cow;

use serde::{Serialize, de::DeserializeOwned};

use crate::{ApiResponse, Client, Operation, RequestOptions, SdkError};

/// A generic client bound to a service and API version, independent of product models.
#[derive(Clone)]
pub struct CommonClient {
    client: Client,
    service: Cow<'static, str>,
    version: Cow<'static, str>,
}

impl CommonClient {
    /// Binds a service and API version while sharing the client's resources.
    ///
    /// # Errors
    /// Returns a configuration error if the service or version is invalid.
    pub fn new(
        client: Client,
        service: impl Into<Cow<'static, str>>,
        version: impl Into<Cow<'static, str>>,
    ) -> Result<Self, SdkError> {
        let service = service.into();
        let version = version.into();
        Operation::new(service.clone(), version.clone(), "Validate")?;
        Ok(Self {
            client,
            service,
            version,
        })
    }

    /// Makes one POST JSON call with default options.
    ///
    /// # Errors
    /// Returns the corresponding input, credential, signing, transport, or Tencent Cloud response error.
    pub async fn call<I: Serialize + Sync + ?Sized, O: DeserializeOwned>(
        &self,
        action: &str,
        input: &I,
    ) -> Result<ApiResponse<O>, SdkError> {
        self.call_with_options(action, input, &RequestOptions::default())
            .await
    }

    /// Calls with per-request options without mutating the shared client.
    ///
    /// # Errors
    /// Returns the corresponding input, credential, signing, transport, or Tencent Cloud response error.
    pub async fn call_with_options<I: Serialize + Sync + ?Sized, O: DeserializeOwned>(
        &self,
        action: &str,
        input: &I,
        options: &RequestOptions,
    ) -> Result<ApiResponse<O>, SdkError> {
        let operation = Operation::new(
            self.service.clone(),
            self.version.clone(),
            action.to_owned(),
        )?;
        self.client
            .execute_with_options(&operation, input, options)
            .await
    }
}
