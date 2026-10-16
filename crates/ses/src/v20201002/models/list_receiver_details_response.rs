// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use super::ReceiverDetail;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ListReceiverDetailsResponse` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListReceiverDetailsResponse {
    /// Official `TotalCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TotalCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i64>,
    /// Official `Data` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ReceiverDetail>>,
    /// Official `ValidCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ValidCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub valid_count: Option<i64>,
    /// Official `InvalidCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "InvalidCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub invalid_count: Option<i64>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for ListReceiverDetailsResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListReceiverDetailsResponse")
            .finish_non_exhaustive()
    }
}
