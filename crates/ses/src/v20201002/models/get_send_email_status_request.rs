// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `GetSendEmailStatusRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSendEmailStatusRequest {
    /// Official `RequestDate` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "RequestDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_date: Option<String>,
    /// Official `Offset` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Official `Limit` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Official `MessageId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "MessageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Official `ToEmailAddress` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ToEmailAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub to_email_address: Option<String>,
}
impl fmt::Debug for GetSendEmailStatusRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GetSendEmailStatusRequest")
            .finish_non_exhaustive()
    }
}
