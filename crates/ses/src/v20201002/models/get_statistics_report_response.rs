// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use super::Volume;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `GetStatisticsReportResponse` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetStatisticsReportResponse {
    /// Official `DailyVolumes` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "DailyVolumes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub daily_volumes: Option<Vec<Volume>>,
    /// Official `OverallVolume` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "OverallVolume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub overall_volume: Option<Volume>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for GetStatisticsReportResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GetStatisticsReportResponse")
            .finish_non_exhaustive()
    }
}
