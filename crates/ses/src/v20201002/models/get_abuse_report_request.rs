// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `GetAbuseReportRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAbuseReportRequest {
    /// Official `StartTime` field; omitted when unset and accepts JSON null.
    #[serde(rename = "StartTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Official `EndTime` field; omitted when unset and accepts JSON null.
    #[serde(rename = "EndTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Official `Offset` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Official `Limit` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Official `FromDomain` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "FromDomain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_domain: Option<String>,
    /// Official `FromAddress` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "FromAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_address: Option<String>,
    /// Official `Mta` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Mta", default, skip_serializing_if = "Option::is_none")]
    pub mta: Option<String>,
    /// Official `ToAddress` field; omitted when unset and accepts JSON null.
    #[serde(rename = "ToAddress", default, skip_serializing_if = "Option::is_none")]
    pub to_address: Option<String>,
    /// Official `TemplateId` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TemplateId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_id: Option<String>,
}
impl fmt::Debug for GetAbuseReportRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GetAbuseReportRequest")
            .finish_non_exhaustive()
    }
}
