// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use super::Tag;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `CreateSecretRequest` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateSecretRequest {
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
    /// Official `Description` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "Description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Official `KmsKeyId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "KmsKeyId", default, skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// Official `SecretType` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_type: Option<i64>,
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
    /// Official `AdditionalConfig` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "AdditionalConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_config: Option<String>,
    /// Official `Tags` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// Official `KmsHsmClusterId` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "KmsHsmClusterId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kms_hsm_cluster_id: Option<String>,
    /// Official `EncryptType` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EncryptType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encrypt_type: Option<i64>,
}
impl fmt::Debug for CreateSecretRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CreateSecretRequest")
            .finish_non_exhaustive()
    }
}
