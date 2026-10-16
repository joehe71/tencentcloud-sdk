// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `DeleteSecretRequest` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteSecretRequest {
    /// Official `SecretName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_name: Option<String>,
    /// Official `RecoveryWindowInDays` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "RecoveryWindowInDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_window_in_days: Option<i64>,
    /// Official `CleanSSHKey` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "CleanSSHKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clean_ssh_key: Option<bool>,
    /// Official `DeleteMode` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "DeleteMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_mode: Option<i64>,
}
impl fmt::Debug for DeleteSecretRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeleteSecretRequest")
            .finish_non_exhaustive()
    }
}
