// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use super::TagList;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ListEmailIdentitiesRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListEmailIdentitiesRequest {
    /// Official `TagList` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TagList", default, skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<TagList>>,
    /// Official `Limit` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Official `Offset` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}
impl fmt::Debug for ListEmailIdentitiesRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListEmailIdentitiesRequest")
            .finish_non_exhaustive()
    }
}
