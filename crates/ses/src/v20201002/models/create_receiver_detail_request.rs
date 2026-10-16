// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `CreateReceiverDetailRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateReceiverDetailRequest {
    /// Official `ReceiverId` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ReceiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub receiver_id: Option<i64>,
    /// Official `Emails` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Emails", default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
}
impl fmt::Debug for CreateReceiverDetailRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CreateReceiverDetailRequest")
            .finish_non_exhaustive()
    }
}
