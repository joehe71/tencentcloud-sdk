// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `BatchSendEmailResponse` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchSendEmailResponse {
    /// Official `TaskId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TaskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<i64>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for BatchSendEmailResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BatchSendEmailResponse")
            .finish_non_exhaustive()
    }
}
