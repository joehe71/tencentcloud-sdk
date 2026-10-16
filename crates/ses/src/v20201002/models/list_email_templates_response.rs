// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use super::TemplatesMetadata;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ListEmailTemplatesResponse` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListEmailTemplatesResponse {
    /// Official `TemplatesMetadata` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TemplatesMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub templates_metadata: Option<Vec<TemplatesMetadata>>,
    /// Official `TotalCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TotalCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i64>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for ListEmailTemplatesResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListEmailTemplatesResponse")
            .finish_non_exhaustive()
    }
}
