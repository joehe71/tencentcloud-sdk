// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ListSendTasksRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListSendTasksRequest {
    /// Official `Offset` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Official `Limit` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Official `Status` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Official `ReceiverId` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ReceiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub receiver_id: Option<i64>,
    /// Official `TaskType` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TaskType", default, skip_serializing_if = "Option::is_none")]
    pub task_type: Option<i64>,
}
impl fmt::Debug for ListSendTasksRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListSendTasksRequest")
            .finish_non_exhaustive()
    }
}
