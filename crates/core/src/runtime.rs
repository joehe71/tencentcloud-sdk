use std::time::{SystemTime, UNIX_EPOCH};

use crate::SdkError;

/// A signing time source that allows fixed clocks in tests without changing system time.
pub trait Clock: Send + Sync {
    /// Returns seconds since the Unix epoch.
    ///
    /// # Errors
    /// Returns a signing error if the current time is unavailable or out of range.
    fn unix_timestamp(&self) -> Result<i64, SdkError>;
}

/// The default system UTC clock.
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn unix_timestamp(&self) -> Result<i64, SdkError> {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| SdkError::Signing("system time is before the Unix epoch"))?
            .as_secs();
        i64::try_from(seconds).map_err(|_| SdkError::Signing("system time is out of range"))
    }
}
