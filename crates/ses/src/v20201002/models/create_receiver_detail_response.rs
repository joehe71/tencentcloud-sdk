// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `CreateReceiverDetailResponse` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateReceiverDetailResponse {
    /// Official `TotalCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TotalCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i64>,
    /// Official `ValidCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ValidCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub valid_count: Option<i64>,
    /// Official `TooLongCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TooLongCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub too_long_count: Option<i64>,
    /// Official `EmptyEmailCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EmptyEmailCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub empty_email_count: Option<i64>,
    /// Official `RepeatCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "RepeatCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repeat_count: Option<i64>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for CreateReceiverDetailResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CreateReceiverDetailResponse")
            .finish_non_exhaustive()
    }
}
