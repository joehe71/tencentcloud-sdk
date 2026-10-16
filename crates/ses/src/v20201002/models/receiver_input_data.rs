// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ReceiverInputData` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReceiverInputData {
    /// Official `Email` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Official `TemplateData` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TemplateData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_data: Option<String>,
}
impl fmt::Debug for ReceiverInputData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReceiverInputData").finish_non_exhaustive()
    }
}
