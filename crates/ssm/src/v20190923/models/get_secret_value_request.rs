// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `GetSecretValueRequest` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSecretValueRequest {
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
    /// Official `EncryptionPublicKey` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EncryptionPublicKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_public_key: Option<String>,
    /// Official `EncryptionAlgorithm` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EncryptionAlgorithm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_algorithm: Option<String>,
}
impl fmt::Debug for GetSecretValueRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GetSecretValueRequest")
            .finish_non_exhaustive()
    }
}
