// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ListEmailTemplatesRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListEmailTemplatesRequest {
    /// Official `Limit` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Official `Offset` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}
impl fmt::Debug for ListEmailTemplatesRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListEmailTemplatesRequest")
            .finish_non_exhaustive()
    }
}
