// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `GetStatisticsReportRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetStatisticsReportRequest {
    /// Official `StartDate` field; omitted when unset and accepts JSON null.
    #[serde(rename = "StartDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Official `EndDate` field; omitted when unset and accepts JSON null.
    #[serde(rename = "EndDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Official `Domain` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Official `ReceivingMailboxType` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ReceivingMailboxType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub receiving_mailbox_type: Option<String>,
}
impl fmt::Debug for GetStatisticsReportRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GetStatisticsReportRequest")
            .finish_non_exhaustive()
    }
}
