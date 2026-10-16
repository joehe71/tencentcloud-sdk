// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ListBlackEmailAddressRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListBlackEmailAddressRequest {
    /// Official `StartDate` field; omitted when unset and accepts JSON null.
    #[serde(rename = "StartDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Official `EndDate` field; omitted when unset and accepts JSON null.
    #[serde(rename = "EndDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Official `Limit` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Official `Offset` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Official `EmailAddress` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EmailAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_address: Option<String>,
    /// Official `TaskID` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TaskID", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}
impl fmt::Debug for ListBlackEmailAddressRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListBlackEmailAddressRequest")
            .finish_non_exhaustive()
    }
}
