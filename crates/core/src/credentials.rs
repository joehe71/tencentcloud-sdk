use std::{env, fmt, sync::Arc};

use http::HeaderValue;

use crate::{BoxFuture, SdkError};

/// 不可变的凭证快照。默认格式化始终脱敏。
///
/// 多个签名字段必须来自同一个快照，不能分别读取可刷新的来源。
pub struct Credentials {
    secret_id: String,
    secret_key: String,
    token: Option<HeaderValue>,
}

impl Credentials {
    /// 构造长期凭证，不访问网络。
    ///
    /// # Errors
    /// `SecretId` 或 `SecretKey` 为空、`SecretId` 不符合 Header 安全字符要求时返回错误。
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
            return Err(SdkError::Credentials("SecretId 或 SecretKey 缺失或无效"));
        }
        Ok(Self {
            secret_id,
            secret_key,
            token: None,
        })
    }

    /// 添加临时凭证 Token。
    ///
    /// # Errors
    /// Token 为空或无法作为 HTTP Header 值时返回错误。
    pub fn with_token(mut self, token: impl AsRef<str>) -> Result<Self, SdkError> {
        if token.as_ref().is_empty() {
            return Err(SdkError::Credentials("临时凭证 Token 不能为空"));
        }
        let mut token = HeaderValue::from_str(token.as_ref())
            .map_err(|_| SdkError::Credentials("临时凭证 Token 无效"))?;
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

/// 可替换的异步凭证来源。实现者负责刷新、缓存和有效期检查。
pub trait ProvideCredentials: Send + Sync {
    /// 返回当前有效的一致快照；失败不得输出凭证原文。
    fn provide_credentials(&self) -> BoxFuture<'_, Result<Arc<Credentials>, SdkError>>;
}

/// 共享一个静态快照，不逐请求复制密钥字符串。
#[derive(Clone, Debug)]
pub struct StaticCredentialsProvider(Arc<Credentials>);

impl StaticCredentialsProvider {
    /// 包装调用方显式提供的凭证。
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

/// 在显式加载时读取环境变量，此后只返回共享快照。
#[derive(Clone, Debug)]
pub struct EnvironmentCredentialsProvider(StaticCredentialsProvider);

impl EnvironmentCredentialsProvider {
    /// 读取 `TENCENTCLOUD_SECRET_ID`、`TENCENTCLOUD_SECRET_KEY` 和可选的 `TENCENTCLOUD_TOKEN`。
    ///
    /// # Errors
    /// 缺少必要变量、变量不是有效 Unicode 或凭证无效时返回错误。
    pub fn load() -> Result<Self, SdkError> {
        Self::from_lookup(|name| match env::var(name) {
            Ok(value) => Ok(Some(value)),
            Err(env::VarError::NotPresent) => Ok(None),
            Err(env::VarError::NotUnicode(_)) => {
                Err(SdkError::Credentials("环境凭证不是有效 Unicode"))
            }
        })
    }

    fn from_lookup(
        mut lookup: impl FnMut(&str) -> Result<Option<String>, SdkError>,
    ) -> Result<Self, SdkError> {
        let id = lookup("TENCENTCLOUD_SECRET_ID")?
            .ok_or(SdkError::Credentials("缺少 TENCENTCLOUD_SECRET_ID"))?;
        let key = lookup("TENCENTCLOUD_SECRET_KEY")?
            .ok_or(SdkError::Credentials("缺少 TENCENTCLOUD_SECRET_KEY"))?;
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
