use std::{borrow::Cow, time::Duration};

use http::{HeaderMap, HeaderName, HeaderValue};

use crate::SdkError;

/// A validated operation descriptor that can borrow static service fields without a runtime registry.
#[derive(Clone, Debug)]
pub struct Operation {
    service: Cow<'static, str>,
    version: Cow<'static, str>,
    action: Cow<'static, str>,
}

impl Operation {
    /// Specifies the service, API date version, and official action.
    ///
    /// # Errors
    /// Returns an error for invalid identifiers or a version not in YYYY-MM-DD format.
    pub fn new(
        service: impl Into<Cow<'static, str>>,
        version: impl Into<Cow<'static, str>>,
        action: impl Into<Cow<'static, str>>,
    ) -> Result<Self, SdkError> {
        let operation = Self {
            service: service.into(),
            version: version.into(),
            action: action.into(),
        };
        validate_service(&operation.service)?;
        let version = operation.version.as_bytes();
        if version.len() != 10
            || !version.iter().enumerate().all(|(index, c)| {
                if matches!(index, 4 | 7) {
                    *c == b'-'
                } else {
                    c.is_ascii_digit()
                }
            })
        {
            return Err(SdkError::Configuration(
                "API version must use YYYY-MM-DD format",
            ));
        }
        if operation.action.is_empty()
            || operation.action.len() > 128
            || !operation.action.bytes().all(|c| c.is_ascii_alphanumeric())
            || !operation.action.as_bytes()[0].is_ascii_alphabetic()
        {
            return Err(SdkError::Configuration(
                "action must be a valid ASCII identifier",
            ));
        }
        Ok(operation)
    }

    /// Returns the service identifier, such as cvm.
    #[must_use]
    pub fn service(&self) -> &str {
        &self.service
    }
    /// Returns the API date version, such as 2017-03-12.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
    /// Returns the official action with its original case.
    #[must_use]
    pub fn action(&self) -> &str {
        &self.action
    }
}

pub(crate) fn validate_service(service: &str) -> Result<(), SdkError> {
    if service.is_empty()
        || service.len() > 63
        || service.starts_with('-')
        || service.ends_with('-')
        || !service
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
    {
        return Err(SdkError::Configuration(
            "service identifier must be a lowercase ASCII domain label",
        ));
    }
    Ok(())
}

pub(crate) fn validate_region(region: &str) -> Result<(), SdkError> {
    if region.is_empty()
        || region.len() > 64
        || !region
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'-')
    {
        return Err(SdkError::Configuration("invalid region"));
    }
    Ok(())
}

/// The Tencent Cloud common language parameter.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Language {
    /// Simplified Chinese.
    ZhCn,
    /// English.
    EnUs,
}

impl Language {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::ZhCn => "zh-CN",
            Self::EnUs => "en-US",
        }
    }
}

/// Per-call options that do not mutate the shared client; headers are omitted from debug output.
#[derive(Clone, Default)]
pub struct RequestOptions {
    pub(crate) region: Option<String>,
    pub(crate) timeout: Option<Duration>,
    pub(crate) language: Option<Language>,
    pub(crate) headers: HeaderMap,
}

impl RequestOptions {
    /// Overrides the region for this request; validated before execution.
    #[must_use]
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }
    /// Overrides the call budget, including credentials, sending, and response body collection.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    /// Sets the response language.
    #[must_use]
    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }
    /// Adds a custom header, replacing any previous value with the same name.
    ///
    /// # Errors
    /// Returns an error when overriding protected signing, common-parameter, or transport headers.
    pub fn header(mut self, name: HeaderName, value: HeaderValue) -> Result<Self, SdkError> {
        let name_str = name.as_str();
        if (name_str.starts_with("x-tc-") && name_str != "x-tc-traceid")
            || matches!(
                name_str,
                "authorization"
                    | "host"
                    | "content-type"
                    | "content-length"
                    | "transfer-encoding"
                    | "connection"
                    | "accept-encoding"
                    | "proxy-authorization"
                    | "trailer"
                    | "te"
                    | "upgrade"
            )
        {
            return Err(SdkError::Configuration(
                "protected HTTP headers cannot be overridden",
            ));
        }
        self.headers.insert(name, value);
        Ok(self)
    }
}

impl std::fmt::Debug for RequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestOptions")
            .field("region", &self.region)
            .field("timeout", &self.timeout)
            .field("language", &self.language)
            .finish_non_exhaustive()
    }
}
