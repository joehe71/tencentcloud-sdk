// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `CreateSSHKeyPairSecretResponse` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateSshKeyPairSecretResponse {
    /// Official `SecretName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_name: Option<String>,
    /// Official `SSHKeyID` field; omitted when unset and accepts JSON null.
    #[serde(rename = "SSHKeyID", default, skip_serializing_if = "Option::is_none")]
    pub ssh_key_id: Option<String>,
    /// Official `SSHKeyName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SSHKeyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ssh_key_name: Option<String>,
    /// Official `TagCode` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TagCode", default, skip_serializing_if = "Option::is_none")]
    pub tag_code: Option<i64>,
    /// Official `TagMsg` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TagMsg", default, skip_serializing_if = "Option::is_none")]
    pub tag_msg: Option<String>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for CreateSshKeyPairSecretResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CreateSshKeyPairSecretResponse")
            .finish_non_exhaustive()
    }
}
