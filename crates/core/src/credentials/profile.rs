use std::{
    env, io,
    path::{Path, PathBuf},
    sync::Arc,
};

use tokio::{fs::File, io::AsyncReadExt};

use super::{Credentials, ProvideCredentials, StaticCredentialsProvider};
use crate::{BoxFuture, SdkError};

pub(super) const FILE_LIMIT: usize = 64 * 1024;

/// A snapshot loaded explicitly from an INI credentials file; files are not watched.
#[derive(Clone, Debug)]
pub struct ProfileCredentialsProvider(StaticCredentialsProvider);

impl ProfileCredentialsProvider {
    /// Loads the default section from one file without consulting other sources.
    ///
    /// # Errors
    /// Returns an error for unreadable, oversized, or invalid files.
    pub async fn load(path: impl AsRef<Path>) -> Result<Self, SdkError> {
        Self::try_load(path.as_ref())
            .await?
            .ok_or(SdkError::Credentials("credentials file not found"))
    }

    /// Searches the Java-compatible system and home paths, taking the first existing file.
    ///
    /// # Errors
    /// Returns an error if no file exists or the first existing file is invalid.
    pub async fn load_default() -> Result<Self, SdkError> {
        Self::discover()
            .await?
            .ok_or(SdkError::Credentials("no default credentials file found"))
    }

    pub(super) async fn discover() -> Result<Option<Self>, SdkError> {
        for path in default_paths() {
            if let Some(provider) = Self::try_load(&path).await? {
                return Ok(Some(provider));
            }
        }
        Ok(None)
    }

    pub(super) async fn try_load(path: &Path) -> Result<Option<Self>, SdkError> {
        let Some(text) = read_file(path).await? else {
            return Ok(None);
        };
        let credentials = parse(&text)?;
        Ok(Some(Self(StaticCredentialsProvider::new(credentials))))
    }
}

impl ProvideCredentials for ProfileCredentialsProvider {
    fn provide_credentials(&self) -> BoxFuture<'_, Result<Arc<Credentials>, SdkError>> {
        self.0.provide_credentials()
    }
}

pub(super) fn default_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    #[cfg(windows)]
    if let Some(home) = env::var_os("USERPROFILE") {
        paths.push(
            PathBuf::from(home)
                .join(".tencentcloud")
                .join("credentials"),
        );
    }
    paths.push(PathBuf::from("/etc/tencentcloud/credentials"));
    if let Some(home) = env::var_os("HOME") {
        paths.push(
            PathBuf::from(home)
                .join(".tencentcloud")
                .join("credentials"),
        );
    }
    paths
}

pub(super) async fn read_file(path: &Path) -> Result<Option<String>, SdkError> {
    let file = match File::open(path).await {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(_) => {
            return Err(SdkError::Credentials(
                "cannot open credentials or identity token file",
            ));
        }
    };
    let mut bytes = Vec::new();
    file.take(u64::try_from(FILE_LIMIT + 1).unwrap_or(u64::MAX))
        .read_to_end(&mut bytes)
        .await
        .map_err(|_| SdkError::Credentials("cannot read credentials or identity token file"))?;
    if bytes.len() > FILE_LIMIT {
        return Err(SdkError::BodyTooLarge { limit: FILE_LIMIT });
    }
    String::from_utf8(bytes)
        .map(Some)
        .map_err(|_| SdkError::Credentials("credentials or identity token file is not UTF-8"))
}

fn parse(text: &str) -> Result<Credentials, SdkError> {
    let mut in_default = false;
    let (mut id, mut key, mut token) = (None, None, None);
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if line.starts_with('[') {
            if !line.ends_with(']') {
                return Err(SdkError::Credentials("invalid credentials section"));
            }
            in_default = line == "[default]";
            continue;
        }
        if !in_default {
            continue;
        }
        let (name, value) = line
            .split_once('=')
            .ok_or(SdkError::Credentials("invalid credentials entry"))?;
        let slot = match name.trim() {
            "secret_id" => &mut id,
            "secret_key" => &mut key,
            "token" => &mut token,
            _ => continue,
        };
        if slot.replace(value.trim()).is_some() {
            return Err(SdkError::Credentials("duplicate credentials entry"));
        }
    }
    let mut credentials = Credentials::new(
        id.ok_or(SdkError::Credentials("missing default.secret_id"))?,
        key.ok_or(SdkError::Credentials("missing default.secret_key"))?,
    )?;
    if let Some(token) = token {
        credentials = credentials.with_token(token)?;
    }
    Ok(credentials)
}

#[cfg(test)]
#[path = "../tests/profile.rs"]
mod tests;
