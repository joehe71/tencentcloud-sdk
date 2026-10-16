// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `GetSSHKeyPairValueRequest` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSshKeyPairValueRequest {
    /// Official `SecretName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_name: Option<String>,
    /// Official `SSHKeyId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "SSHKeyId", default, skip_serializing_if = "Option::is_none")]
    pub ssh_key_id: Option<String>,
}
impl fmt::Debug for GetSshKeyPairValueRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GetSshKeyPairValueRequest")
            .finish_non_exhaustive()
    }
}
