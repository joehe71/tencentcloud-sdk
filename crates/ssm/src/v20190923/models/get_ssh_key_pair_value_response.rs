// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `GetSSHKeyPairValueResponse` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSshKeyPairValueResponse {
    /// Official `SSHKeyID` field; omitted when unset and accepts JSON null.
    #[serde(rename = "SSHKeyID", default, skip_serializing_if = "Option::is_none")]
    pub ssh_key_id: Option<String>,
    /// Official `PublicKey` field; omitted when unset and accepts JSON null.
    #[serde(rename = "PublicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// Official `PrivateKey` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "PrivateKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub private_key: Option<String>,
    /// Official `ProjectID` field; omitted when unset and accepts JSON null.
    #[serde(rename = "ProjectID", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    /// Official `SSHKeyDescription` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SSHKeyDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ssh_key_description: Option<String>,
    /// Official `SSHKeyName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SSHKeyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ssh_key_name: Option<String>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for GetSshKeyPairValueResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GetSshKeyPairValueResponse")
            .finish_non_exhaustive()
    }
}
