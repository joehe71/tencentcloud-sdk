use std::{
    env, fmt,
    path::PathBuf,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;

use super::{
    Credentials, ProvideCredentials,
    cache::{LoadCredentials, RefreshingProvider},
    profile::read_file,
    role_http::{CredentialHttp, ExpiringCredentials, RoleProviderOptions, validate_role_name},
};
use crate::{BoxFuture, SdkError, operation::validate_region};

/// An OIDC token supplied in memory or reloaded from a rotating projected file.
pub enum WebIdentityToken {
    /// A caller-managed token; the caller is responsible for rotation.
    Inline(String),
    /// A UTF-8 token file re-opened on every credential refresh, including symlink rotation.
    File(PathBuf),
}
impl fmt::Debug for WebIdentityToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WebIdentityToken { .. }")
    }
}

/// Configuration shared by signed STS role sessions.
pub struct AssumeRoleConfig {
    region: String,
    role_arn: String,
    session_name: String,
    duration_seconds: u64,
}
impl AssumeRoleConfig {
    /// Sets a region and role ARN with a generated session name and a two-hour lifetime.
    #[must_use]
    pub fn new(region: impl Into<String>, role_arn: impl Into<String>) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        Self {
            region: region.into(),
            role_arn: role_arn.into(),
            session_name: format!("tencentcloud-rust-sdk-{nanos}"),
            duration_seconds: 7200,
        }
    }
    /// Sets a session name for audit logs; validated when constructing a provider.
    #[must_use]
    pub fn session_name(mut self, name: impl Into<String>) -> Self {
        self.session_name = name.into();
        self
    }
    /// Requests a lifetime in seconds; validated against STS's [900, 43200] range.
    #[must_use]
    pub fn duration_seconds(mut self, seconds: u64) -> Self {
        self.duration_seconds = seconds;
        self
    }
    fn validate(&self) -> Result<(), SdkError> {
        validate_region(&self.region)?;
        validate_identifier(&self.role_arn)?;
        if !(2..=128).contains(&self.session_name.len())
            || !self
                .session_name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"_+=,.@-".contains(&byte))
            || !(900..=43_200).contains(&self.duration_seconds)
        {
            return Err(SdkError::Configuration(
                "invalid role session name or duration",
            ));
        }
        Ok(())
    }
}
impl fmt::Debug for AssumeRoleConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AssumeRoleConfig").finish_non_exhaustive()
    }
}

/// Configuration for STS `AssumeRoleWithWebIdentity`, independent of TKE environment variables.
pub struct OidcRoleConfig {
    session: AssumeRoleConfig,
    provider_id: String,
    token: WebIdentityToken,
}
impl OidcRoleConfig {
    /// Sets the region, OIDC provider, role ARN, and token source.
    #[must_use]
    pub fn new(
        region: impl Into<String>,
        provider_id: impl Into<String>,
        role_arn: impl Into<String>,
        token: WebIdentityToken,
    ) -> Self {
        Self {
            session: AssumeRoleConfig::new(region, role_arn),
            provider_id: provider_id.into(),
            token,
        }
    }
    /// Sets a session name for audit logs.
    #[must_use]
    pub fn session_name(mut self, name: impl Into<String>) -> Self {
        self.session = self.session.session_name(name);
        self
    }
    /// Requests a lifetime in seconds, subject to the role's server-side maximum.
    #[must_use]
    pub fn duration_seconds(mut self, seconds: u64) -> Self {
        self.session = self.session.duration_seconds(seconds);
        self
    }
    pub(super) fn from_lookup(
        mut lookup: impl FnMut(&str) -> Result<Option<String>, SdkError>,
    ) -> Result<Self, SdkError> {
        let mut required = |name| {
            lookup(name)?
                .filter(|value| !value.is_empty())
                .ok_or(SdkError::Credentials(
                    "missing required TKE OIDC environment variable",
                ))
        };
        let region = required("TKE_REGION")?;
        let provider_id = required("TKE_PROVIDER_ID")?;
        let role_arn = required("TKE_ROLE_ARN")?;
        let token = WebIdentityToken::File(PathBuf::from(required("TKE_WEB_IDENTITY_TOKEN_FILE")?));
        Ok(Self::new(region, provider_id, role_arn, token))
    }
}
impl fmt::Debug for OidcRoleConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OidcRoleConfig").finish_non_exhaustive()
    }
}

/// Cached OIDC role credentials, with single-flight refresh and projected-token rotation.
#[derive(Clone)]
pub struct OidcRoleCredentialsProvider(Arc<RefreshingProvider>);
impl OidcRoleCredentialsProvider {
    /// Creates a lazy provider without reading a token file or contacting STS.
    ///
    /// # Errors
    /// Returns an error for invalid role, session, token-source, or HTTP options.
    pub fn new(config: OidcRoleConfig, options: RoleProviderOptions) -> Result<Self, SdkError> {
        config.session.validate()?;
        validate_identifier(&config.provider_id)?;
        if matches!(&config.token, WebIdentityToken::Inline(token) if token.trim().is_empty() || token.len() > super::profile::FILE_LIMIT)
            || matches!(&config.token, WebIdentityToken::File(path) if path.as_os_str().is_empty())
        {
            return Err(SdkError::Configuration("invalid web identity token source"));
        }
        let http = CredentialHttp::new(options.clone())?;
        Ok(Self(Arc::new(RefreshingProvider::new(
            OidcSource { config, http },
            options,
        ))))
    }
    /// Reads `TKE_REGION`, `TKE_PROVIDER_ID`, `TKE_ROLE_ARN`, and `TKE_WEB_IDENTITY_TOKEN_FILE`.
    ///
    /// # Errors
    /// Returns an error for missing/invalid environment variables or invalid configuration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[cfg(feature = "transport-reqwest")]
    /// # fn build_client() -> Result<tencentcloud_sdk_core::Client, tencentcloud_sdk_core::SdkError> {
    /// use std::{sync::Arc, time::Duration};
    /// use tencentcloud_sdk_core::{Client, credentials::{OidcRoleCredentialsProvider, RoleProviderOptions},
    ///     transport::ReqwestTransport};
    /// let transport = Arc::new(ReqwestTransport::new(Duration::from_secs(10))?);
    /// let provider = OidcRoleCredentialsProvider::from_tke_env(
    ///     RoleProviderOptions::new(transport.clone()),
    /// )?;
    /// Client::builder().credentials_provider(provider).shared_transport(transport)
    ///     .region("ap-shanghai").build()
    /// # }
    /// ```
    pub fn from_tke_env(options: RoleProviderOptions) -> Result<Self, SdkError> {
        Self::new(OidcRoleConfig::from_lookup(environment)?, options)
    }
}
impl ProvideCredentials for OidcRoleCredentialsProvider {
    fn provide_credentials(&self) -> BoxFuture<'_, Result<Arc<Credentials>, SdkError>> {
        self.0.provide()
    }
}

struct OidcSource {
    config: OidcRoleConfig,
    http: CredentialHttp,
}
impl LoadCredentials for OidcSource {
    fn load(&self) -> BoxFuture<'_, Result<ExpiringCredentials, SdkError>> {
        Box::pin(async {
            let token = match &self.config.token {
                WebIdentityToken::Inline(token) => token.clone(),
                WebIdentityToken::File(path) => read_file(path)
                    .await?
                    .ok_or(SdkError::Credentials("web identity token file not found"))?,
            };
            let token = token.trim();
            if token.is_empty() {
                return Err(SdkError::Credentials("web identity token is empty"));
            }
            let session = &self.config.session;
            let input = OidcRequest {
                provider_id: &self.config.provider_id,
                web_identity_token: token,
                role_arn: &session.role_arn,
                role_session_name: &session.session_name,
                duration_seconds: session.duration_seconds,
            };
            self.http
                .sts("AssumeRoleWithWebIdentity", &session.region, &input, None)
                .await
        })
    }
}
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct OidcRequest<'a> {
    provider_id: &'a str,
    web_identity_token: &'a str,
    role_arn: &'a str,
    role_session_name: &'a str,
    duration_seconds: u64,
}

/// Cached CVM instance-role credentials, fetched from the metadata server, not STS.
#[derive(Clone)]
pub struct CvmRoleCredentialsProvider(Arc<RefreshingProvider>);
impl CvmRoleCredentialsProvider {
    /// Uses the specified metadata role name, or discovers it when None.
    ///
    /// # Errors
    /// Returns an error for an invalid role name or HTTP options; construction performs no I/O.
    pub fn new(role_name: Option<String>, options: RoleProviderOptions) -> Result<Self, SdkError> {
        if let Some(role) = &role_name {
            validate_role_name(role)?;
        }
        let http = CredentialHttp::new(options.clone())?;
        Ok(Self(Arc::new(RefreshingProvider::new(
            CvmSource { role_name, http },
            options,
        ))))
    }
}
impl ProvideCredentials for CvmRoleCredentialsProvider {
    fn provide_credentials(&self) -> BoxFuture<'_, Result<Arc<Credentials>, SdkError>> {
        self.0.provide()
    }
}
struct CvmSource {
    role_name: Option<String>,
    http: CredentialHttp,
}
impl LoadCredentials for CvmSource {
    fn load(&self) -> BoxFuture<'_, Result<ExpiringCredentials, SdkError>> {
        Box::pin(async { self.http.metadata(self.role_name.as_deref()).await })
    }
}

/// Assumes a role using a replaceable, possibly refreshable source credentials provider.
#[derive(Clone)]
pub struct AssumeRoleCredentialsProvider(Arc<RefreshingProvider>);
impl AssumeRoleCredentialsProvider {
    /// Creates a lazy signed STS `AssumeRole` provider; temporary source tokens are retained.
    ///
    /// # Errors
    /// Returns an error for invalid session configuration or HTTP options.
    pub fn new(
        source: Arc<dyn ProvideCredentials>,
        config: AssumeRoleConfig,
        options: RoleProviderOptions,
    ) -> Result<Self, SdkError> {
        config.validate()?;
        let http = CredentialHttp::new(options.clone())?;
        Ok(Self(Arc::new(RefreshingProvider::new(
            AssumeSource {
                source,
                config,
                http,
            },
            options,
        ))))
    }
}
impl ProvideCredentials for AssumeRoleCredentialsProvider {
    fn provide_credentials(&self) -> BoxFuture<'_, Result<Arc<Credentials>, SdkError>> {
        self.0.provide()
    }
}
struct AssumeSource {
    source: Arc<dyn ProvideCredentials>,
    config: AssumeRoleConfig,
    http: CredentialHttp,
}
impl LoadCredentials for AssumeSource {
    fn load(&self) -> BoxFuture<'_, Result<ExpiringCredentials, SdkError>> {
        Box::pin(async {
            let signer = self.source.provide_credentials().await?;
            let input = AssumeRequest {
                role_arn: &self.config.role_arn,
                role_session_name: &self.config.session_name,
                duration_seconds: self.config.duration_seconds,
            };
            self.http
                .sts("AssumeRole", &self.config.region, &input, Some(&signer))
                .await
        })
    }
}
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct AssumeRequest<'a> {
    role_arn: &'a str,
    role_session_name: &'a str,
    duration_seconds: u64,
}

pub(super) fn environment(name: &str) -> Result<Option<String>, SdkError> {
    match env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => Err(SdkError::Credentials(
            "credential environment value is not valid Unicode",
        )),
    }
}
fn validate_identifier(value: &str) -> Result<(), SdkError> {
    if value.is_empty() || value.len() > 2048 || value.chars().any(char::is_control) {
        return Err(SdkError::Configuration(
            "invalid role ARN or OIDC provider identifier",
        ));
    }
    Ok(())
}

macro_rules! redacted_debug {
    ($($name:ident),+ $(,)?) => { $(impl fmt::Debug for $name {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct(stringify!($name)).finish_non_exhaustive() }
    })+ };
}
redacted_debug!(
    OidcRoleCredentialsProvider,
    CvmRoleCredentialsProvider,
    AssumeRoleCredentialsProvider
);
