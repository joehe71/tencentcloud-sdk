use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use tokio::{
    sync::Mutex,
    time::{Instant, timeout},
};

use super::{
    Credentials,
    role_http::{ExpiringCredentials, RoleProviderOptions},
};
use crate::{BoxFuture, SdkError};

pub(super) trait LoadCredentials: Send + Sync {
    fn load(&self) -> BoxFuture<'_, Result<ExpiringCredentials, SdkError>>;
}

struct Snapshot {
    value: ExpiringCredentials,
    refresh_at: i64,
}
#[derive(Default)]
struct State {
    snapshot: Option<Snapshot>,
    retry_at: Option<Instant>,
}

pub(super) struct RefreshingProvider {
    source: Arc<dyn LoadCredentials>,
    options: RoleProviderOptions,
    state: RwLock<State>,
    refresh: Mutex<()>,
}

impl RefreshingProvider {
    pub fn new(source: impl LoadCredentials + 'static, options: RoleProviderOptions) -> Self {
        Self {
            source: Arc::new(source),
            options,
            state: RwLock::new(State::default()),
            refresh: Mutex::new(()),
        }
    }

    pub fn provide(&self) -> BoxFuture<'_, Result<Arc<Credentials>, SdkError>> {
        Box::pin(async {
            timeout(self.options.timeout, self.resolve())
                .await
                .map_err(|_| SdkError::Timeout)?
        })
    }

    fn cached(&self, now: i64) -> Result<Option<Arc<Credentials>>, SdkError> {
        let state = self
            .state
            .read()
            .map_err(|_| SdkError::Credentials("credential cache lock is poisoned"))?;
        if let Some(snapshot) = &state.snapshot {
            if now < snapshot.refresh_at && now < snapshot.value.expires_at {
                return Ok(Some(Arc::clone(&snapshot.value.credentials)));
            }
        }
        if state
            .retry_at
            .is_some_and(|retry_at| Instant::now() < retry_at)
        {
            return Err(SdkError::Credentials(
                "credential refresh is in failure backoff",
            ));
        }
        Ok(None)
    }

    async fn resolve(&self) -> Result<Arc<Credentials>, SdkError> {
        if let Some(credentials) = self.cached(self.options.clock.unix_timestamp()?)? {
            return Ok(credentials);
        }
        // Only the asynchronous refresh gate spans await points. Snapshot locks never do.
        let _guard = self.refresh.lock().await;
        if let Some(credentials) = self.cached(self.options.clock.unix_timestamp()?)? {
            return Ok(credentials);
        }
        // The outer call budget also covers this I/O; cancellation releases the refresh gate.
        let result = self.source.load().await.and_then(|value| {
            let now = self.options.clock.unix_timestamp()?;
            if value.expires_at <= now {
                return Err(SdkError::Credentials(
                    "provider returned expired credentials",
                ));
            }
            let lifetime = value
                .expires_at
                .checked_sub(now)
                .ok_or(SdkError::Credentials("credential lifetime is out of range"))?;
            let window = i64::try_from(self.options.refresh_before.as_secs())
                .map_err(|_| SdkError::Configuration("refresh window is out of range"))?
                .min(lifetime / 2);
            Ok(Snapshot {
                refresh_at: value.expires_at - window,
                value,
            })
        });
        let mut state = self
            .state
            .write()
            .map_err(|_| SdkError::Credentials("credential cache lock is poisoned"))?;
        match result {
            Ok(snapshot) => {
                let credentials = Arc::clone(&snapshot.value.credentials);
                state.snapshot = Some(snapshot);
                state.retry_at = None;
                Ok(credentials)
            }
            Err(error) => {
                // Do not silently use stale credentials or repeatedly hit STS under contention.
                state.retry_at = Some(Instant::now() + Duration::from_secs(5));
                Err(error)
            }
        }
    }
}
