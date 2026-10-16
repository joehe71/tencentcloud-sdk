// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `RotateProductSecretRequest` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RotateProductSecretRequest {
    /// Official `SecretName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_name: Option<String>,
}
impl fmt::Debug for RotateProductSecretRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RotateProductSecretRequest")
            .finish_non_exhaustive()
    }
}
