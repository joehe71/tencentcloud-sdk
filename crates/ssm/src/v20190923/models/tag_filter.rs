// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `TagFilter` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TagFilter {
    /// Official `TagKey` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TagKey", default, skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// Official `TagValue` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TagValue", default, skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<Vec<String>>,
}
impl fmt::Debug for TagFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TagFilter").finish_non_exhaustive()
    }
}
