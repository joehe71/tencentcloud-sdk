// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `DescribeRotationDetailResponse` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DescribeRotationDetailResponse {
    /// Official `EnableRotation` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EnableRotation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_rotation: Option<bool>,
    /// Official `Frequency` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Frequency", default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    /// Official `LatestRotateTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "LatestRotateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_rotate_time: Option<String>,
    /// Official `NextRotateBeginTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "NextRotateBeginTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_rotate_begin_time: Option<String>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for DescribeRotationDetailResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DescribeRotationDetailResponse")
            .finish_non_exhaustive()
    }
}
