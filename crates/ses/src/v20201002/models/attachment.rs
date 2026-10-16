// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `Attachment` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Attachment {
    /// Official `FileName` field; omitted when unset and accepts JSON null.
    #[serde(rename = "FileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Official `Content` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Content", default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Official `FileURL` field; omitted when unset and accepts JSON null.
    #[serde(rename = "FileURL", default, skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
}
impl fmt::Debug for Attachment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Attachment").finish_non_exhaustive()
    }
}
