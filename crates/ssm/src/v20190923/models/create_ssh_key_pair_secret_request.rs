// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use super::Tag;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `CreateSSHKeyPairSecretRequest` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateSshKeyPairSecretRequest {
    /// Official `SecretName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_name: Option<String>,
    /// Official `ProjectId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "ProjectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
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
    /// Official `Tags` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// Official `SSHKeyName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SSHKeyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ssh_key_name: Option<String>,
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
impl fmt::Debug for CreateSshKeyPairSecretRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CreateSshKeyPairSecretRequest")
            .finish_non_exhaustive()
    }
}
