use std::{borrow::Cow, time::Duration};

use http::{HeaderMap, HeaderName, HeaderValue};

use crate::SdkError;

/// 经过验证的操作描述。静态产品字段可借用字符串字面量，无运行时注册表。
#[derive(Clone, Debug)]
pub struct Operation {
    service: Cow<'static, str>,
    version: Cow<'static, str>,
    action: Cow<'static, str>,
}

impl Operation {
    /// 指定产品、API 日期版本和官方 Action。
    ///
    /// # Errors
    /// 标识含非法字符或版本不是 YYYY-MM-DD 格式时返回错误。
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
            return Err(SdkError::Configuration("API 版本必须使用 YYYY-MM-DD 格式"));
        }
        if operation.action.is_empty()
            || operation.action.len() > 128
            || !operation.action.bytes().all(|c| c.is_ascii_alphanumeric())
            || !operation.action.as_bytes()[0].is_ascii_alphabetic()
        {
            return Err(SdkError::Configuration("Action 必须是有效的 ASCII 标识"));
        }
        Ok(operation)
    }

    /// 产品标识，例如 cvm。
    #[must_use]
    pub fn service(&self) -> &str {
        &self.service
    }
    /// API 日期版本，例如 2017-03-12。
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
    /// 官方 Action，保留大小写。
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
        return Err(SdkError::Configuration("产品标识必须为小写 ASCII 域名标签"));
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
        return Err(SdkError::Configuration("Region 无效"));
    }
    Ok(())
}

/// 腾讯云公共语言参数。
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Language {
    /// 简体中文。
    ZhCn,
    /// 英文。
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

/// 单次调用选项，不修改共享 Client。Header 的调试输出保持脱敏。
#[derive(Clone, Default)]
pub struct RequestOptions {
    pub(crate) region: Option<String>,
    pub(crate) timeout: Option<Duration>,
    pub(crate) language: Option<Language>,
    pub(crate) headers: HeaderMap,
}

impl RequestOptions {
    /// 覆盖本次请求的 Region；在执行前验证。
    #[must_use]
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }
    /// 覆盖本次调用预算，覆盖凭证获取、发送及响应体读取。
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    /// 指定响应语言。
    #[must_use]
    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }
    /// 增加自定义 Header；同名 Header 后值覆盖前值。
    ///
    /// # Errors
    /// 尝试覆盖签名、公共参数、传输编码等受保护 Header 时返回错误。
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
            return Err(SdkError::Configuration("不能覆盖受保护的 HTTP Header"));
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
