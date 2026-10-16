use std::{env, fmt, sync::Arc};

use http::HeaderValue;

use crate::{BoxFuture, SdkError};

#[cfg(feature = "credentials-file")]
mod profile;
#[cfg(feature = "credentials-file")]
pub use profile::ProfileCredentialsProvider;

#[cfg(feature = "credentials-roles")]
mod cache;
#[cfg(feature = "credentials-roles")]
mod chain;
#[cfg(feature = "credentials-roles")]
mod role_http;
#[cfg(feature = "credentials-roles")]
mod roles;
#[cfg(feature = "credentials-roles")]
pub use chain::DefaultCredentialsProvider;
#[cfg(feature = "credentials-roles")]
pub use role_http::RoleProviderOptions;
#[cfg(feature = "credentials-roles")]
pub use roles::{
    AssumeRoleConfig, AssumeRoleCredentialsProvider, CvmRoleCredentialsProvider, OidcRoleConfig,
    OidcRoleCredentialsProvider, WebIdentityToken,
};

/// An immutable credentials snapshot with redacted default formatting.
///
/// All signing fields must come from one snapshot, not separate reads of a refreshable source.
pub struct Credentials {
    secret_id: String,
    secret_key: String,
    token: Option<HeaderValue>,
}

impl Credentials {
    /// Creates long-lived credentials without network access.
    ///
    /// # Errors
    /// Returns an error if either secret is empty or `SecretId` contains characters unsafe for headers.
    pub fn new(
        secret_id: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> Result<Self, SdkError> {
        let secret_id = secret_id.into();
        let secret_key = secret_key.into();
        if secret_id.is_empty()
            || !secret_id
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-'))
            || secret_key.is_empty()
        {
            return Err(SdkError::Credentials(
                "SecretId or SecretKey is missing or invalid",
            ));
        }
        Ok(Self {
            secret_id,
            secret_key,
            token: None,
        })
    }

    /// Adds a temporary credentials token.
    ///
    /// # Errors
    /// Returns an error if the token is empty or invalid as an HTTP header value.
    pub fn with_token(mut self, token: impl AsRef<str>) -> Result<Self, SdkError> {
        if token.as_ref().is_empty() {
            return Err(SdkError::Credentials(
                "temporary credentials token must not be empty",
            ));
        }
        let mut token = HeaderValue::from_str(token.as_ref())
            .map_err(|_| SdkError::Credentials("invalid temporary credentials token"))?;
        token.set_sensitive(true);
        self.token = Some(token);
        Ok(self)
    }

    pub(crate) fn secret_id(&self) -> &str {
        &self.secret_id
    }
    pub(crate) fn secret_key(&self) -> &str {
        &self.secret_key
    }
    pub(crate) fn token(&self) -> Option<&HeaderValue> {
        self.token.as_ref()
    }
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials").finish_non_exhaustive()
    }
}

/// A replaceable asynchronous credentials source responsible for refresh, caching, and expiration checks.
pub trait ProvideCredentials: Send + Sync {
    /// Returns a consistent, currently valid snapshot; failures must not expose credentials.
    fn provide_credentials(&self) -> BoxFuture<'_, Result<Arc<Credentials>, SdkError>>;
}

/// Shares a static snapshot without copying secret strings for each request.
#[derive(Clone, Debug)]
pub struct StaticCredentialsProvider(Arc<Credentials>);

impl StaticCredentialsProvider {
    /// Wraps credentials explicitly supplied by the caller.
    #[must_use]
    pub fn new(credentials: Credentials) -> Self {
        Self(Arc::new(credentials))
    }
}

impl ProvideCredentials for StaticCredentialsProvider {
    fn provide_credentials(&self) -> BoxFuture<'_, Result<Arc<Credentials>, SdkError>> {
        Box::pin(async { Ok(Arc::clone(&self.0)) })
    }
}

/// Reads environment variables on explicit load, then returns a shared snapshot.
#[derive(Clone, Debug)]
pub struct EnvironmentCredentialsProvider(StaticCredentialsProvider);

impl EnvironmentCredentialsProvider {
    /// Loads `TENCENTCLOUD_SECRET_ID`, `TENCENTCLOUD_SECRET_KEY`, and optional `TENCENTCLOUD_TOKEN`.
    ///
    /// # Errors
    /// Returns an error for missing required variables, invalid Unicode, or invalid credentials.
    pub fn load() -> Result<Self, SdkError> {
        Self::from_lookup(|name| match env::var(name) {
            Ok(value) => Ok(Some(value)),
            Err(env::VarError::NotPresent) => Ok(None),
            Err(env::VarError::NotUnicode(_)) => Err(SdkError::Credentials(
                "environment credentials are not valid Unicode",
            )),
        })
    }

    fn from_lookup(
        mut lookup: impl FnMut(&str) -> Result<Option<String>, SdkError>,
    ) -> Result<Self, SdkError> {
        let id = lookup("TENCENTCLOUD_SECRET_ID")?
            .ok_or(SdkError::Credentials("missing TENCENTCLOUD_SECRET_ID"))?;
        let key = lookup("TENCENTCLOUD_SECRET_KEY")?
            .ok_or(SdkError::Credentials("missing TENCENTCLOUD_SECRET_KEY"))?;
        let mut credentials = Credentials::new(id, key)?;
        if let Some(token) = lookup("TENCENTCLOUD_TOKEN")? {
            credentials = credentials.with_token(token)?;
        }
        Ok(Self(StaticCredentialsProvider::new(credentials)))
    }
}

impl ProvideCredentials for EnvironmentCredentialsProvider {
    fn provide_credentials(&self) -> BoxFuture<'_, Result<Arc<Credentials>, SdkError>> {
        self.0.provide_credentials()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn environment_is_loaded_once_without_mutating_process_state() {
        let mut reads = 0;
        let provider = EnvironmentCredentialsProvider::from_lookup(|name| {
            reads += 1;
            Ok(Some(
                match name {
                    "TENCENTCLOUD_SECRET_ID" => "test-id",
                    "TENCENTCLOUD_SECRET_KEY" => "test-key",
                    _ => "test-token",
                }
                .to_owned(),
            ))
        })
        .unwrap();
        let first = provider.provide_credentials().await.unwrap();
        let second = provider.provide_credentials().await.unwrap();
        assert!(Arc::ptr_eq(&first, &second));
        assert_eq!(reads, 3);
        assert_eq!(first.token().unwrap(), "test-token");
        assert!(first.token().unwrap().is_sensitive());
    }

    #[test]
    fn rejects_missing_invalid_and_empty_credentials() {
        assert!(EnvironmentCredentialsProvider::from_lookup(|_| Ok(None)).is_err());
        assert!(Credentials::new("", "test-key").is_err());
        assert!(Credentials::new("id\n", "test-key").is_err());
        assert!(Credentials::new("test-id", "").is_err());
        assert!(
            Credentials::new("test-id", "test-key")
                .unwrap()
                .with_token("")
                .is_err()
        );
        assert!(
            Credentials::new("test-id", "test-key")
                .unwrap()
                .with_token("x\ny")
                .is_err()
        );
    }

    #[test]
    fn debug_does_not_expose_any_secret() {
        let credentials = Credentials::new("private-id", "private-key")
            .unwrap()
            .with_token("private-token")
            .unwrap();
        let debug = format!("{credentials:?}");
        for secret in ["private-id", "private-key", "private-token"] {
            assert!(!debug.contains(secret));
        }
    }
}
