// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use super::{CycleEmailParam, Template, TimedEmailParam};
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `SendTaskData` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SendTaskData {
    /// Official `TaskId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TaskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<i64>,
    /// Official `FromEmailAddress` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "FromEmailAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_email_address: Option<String>,
    /// Official `ReceiverId` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ReceiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub receiver_id: Option<i64>,
    /// Official `TaskStatus` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TaskStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub task_status: Option<i64>,
    /// Official `TaskType` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TaskType", default, skip_serializing_if = "Option::is_none")]
    pub task_type: Option<i64>,
    /// Official `RequestCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "RequestCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_count: Option<i64>,
    /// Official `SendCount` field; omitted when unset and accepts JSON null.
    #[serde(rename = "SendCount", default, skip_serializing_if = "Option::is_none")]
    pub send_count: Option<i64>,
    /// Official `CacheCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "CacheCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cache_count: Option<i64>,
    /// Official `CreateTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "CreateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<String>,
    /// Official `UpdateTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "UpdateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<String>,
    /// Official `Subject` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Official `Template` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Template", default, skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
    /// Official `CycleParam` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "CycleParam",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cycle_param: Option<CycleEmailParam>,
    /// Official `TimedParam` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TimedParam",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timed_param: Option<TimedEmailParam>,
    /// Official `ErrMsg` field; omitted when unset and accepts JSON null.
    #[serde(rename = "ErrMsg", default, skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    /// Official `ReceiversName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ReceiversName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub receivers_name: Option<String>,
}
impl fmt::Debug for SendTaskData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SendTaskData").finish_non_exhaustive()
    }
}
