use std::time::{SystemTime, UNIX_EPOCH};

use crate::SdkError;

/// 签名使用的时间来源。测试可以注入固定时钟，不影响操作系统时间。
pub trait Clock: Send + Sync {
    /// 返回 Unix 秒。
    ///
    /// # Errors
    /// 当前时间不可用或无法表示时返回签名错误。
    fn unix_timestamp(&self) -> Result<i64, SdkError>;
}

/// 默认的系统 UTC 时钟。
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn unix_timestamp(&self) -> Result<i64, SdkError> {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| SdkError::Signing("系统时间早于 Unix epoch"))?
            .as_secs();
        i64::try_from(seconds).map_err(|_| SdkError::Signing("系统时间超出有效范围"))
    }
}
