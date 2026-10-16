// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `Template` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Template {
    /// Official `TemplateID` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TemplateID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_id: Option<i64>,
    /// Official `TemplateData` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TemplateData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_data: Option<String>,
}
impl fmt::Debug for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Template").finish_non_exhaustive()
    }
}
