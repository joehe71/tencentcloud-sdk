// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ReceiverDetail` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReceiverDetail {
    /// Official `Email` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Official `CreateTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "CreateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<String>,
    /// Official `TemplateData` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TemplateData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_data: Option<String>,
    /// Official `Reason` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Official `Status` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Official `EmailId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "EmailId", default, skip_serializing_if = "Option::is_none")]
    pub email_id: Option<i64>,
}
impl fmt::Debug for ReceiverDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReceiverDetail").finish_non_exhaustive()
    }
}
