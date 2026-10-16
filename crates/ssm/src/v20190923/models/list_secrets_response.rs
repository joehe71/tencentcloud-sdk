// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use super::SecretMetadata;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ListSecretsResponse` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListSecretsResponse {
    /// Official `TotalCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TotalCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i64>,
    /// Official `SecretMetadatas` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretMetadatas",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_metadatas: Option<Vec<SecretMetadata>>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for ListSecretsResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListSecretsResponse")
            .finish_non_exhaustive()
    }
}
