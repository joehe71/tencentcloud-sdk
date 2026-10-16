// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `UpdateSecretRequest` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateSecretRequest {
    /// Official `SecretName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_name: Option<String>,
    /// Official `VersionId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "VersionId", default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// Official `SecretBinary` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretBinary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_binary: Option<String>,
    /// Official `SecretString` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretString",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_string: Option<String>,
}
impl fmt::Debug for UpdateSecretRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UpdateSecretRequest")
            .finish_non_exhaustive()
    }
}
