// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ReceiverData` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReceiverData {
    /// Official `ReceiverId` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ReceiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub receiver_id: Option<i64>,
    /// Official `ReceiversName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ReceiversName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub receivers_name: Option<String>,
    /// Official `Count` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Official `Desc` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Desc", default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// Official `ReceiversStatus` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ReceiversStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub receivers_status: Option<i64>,
    /// Official `CreateTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "CreateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<String>,
    /// Official `InvalidCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "InvalidCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub invalid_count: Option<i64>,
}
impl fmt::Debug for ReceiverData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReceiverData").finish_non_exhaustive()
    }
}
