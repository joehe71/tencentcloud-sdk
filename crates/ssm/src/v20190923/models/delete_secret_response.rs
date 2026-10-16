// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `DeleteSecretResponse` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteSecretResponse {
    /// Official `SecretName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_name: Option<String>,
    /// Official `DeleteTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "DeleteTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_time: Option<i64>,
    /// Official `FlowID` field; omitted when unset and accepts JSON null.
    #[serde(rename = "FlowID", default, skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<i64>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for DeleteSecretResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeleteSecretResponse")
            .finish_non_exhaustive()
    }
}
