// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `DeleteReceiverRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteReceiverRequest {
    /// Official `ReceiverId` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ReceiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub receiver_id: Option<i64>,
}
impl fmt::Debug for DeleteReceiverRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeleteReceiverRequest")
            .finish_non_exhaustive()
    }
}
