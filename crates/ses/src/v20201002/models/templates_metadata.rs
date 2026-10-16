// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `TemplatesMetadata` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TemplatesMetadata {
    /// Official `CreatedTimestamp` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "CreatedTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_timestamp: Option<i64>,
    /// Official `TemplateName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TemplateName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_name: Option<String>,
    /// Official `TemplateStatus` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TemplateStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_status: Option<i64>,
    /// Official `TemplateID` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TemplateID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_id: Option<i64>,
    /// Official `ReviewReason` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ReviewReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub review_reason: Option<String>,
}
impl fmt::Debug for TemplatesMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TemplatesMetadata").finish_non_exhaustive()
    }
}
