// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use super::{Attachment, CycleEmailParam, Simple, Template, TimedEmailParam};
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `BatchSendEmailRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchSendEmailRequest {
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
    /// Official `Subject` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Official `TaskType` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TaskType", default, skip_serializing_if = "Option::is_none")]
    pub task_type: Option<i64>,
    /// Official `ReplyToAddresses` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ReplyToAddresses",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_to_addresses: Option<String>,
    /// Official `Template` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Template", default, skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
    /// Official `Simple` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Simple", default, skip_serializing_if = "Option::is_none")]
    pub simple: Option<Simple>,
    /// Official `Attachments` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "Attachments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attachments: Option<Vec<Attachment>>,
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
    /// Official `Unsubscribe` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "Unsubscribe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribe: Option<String>,
    /// Official `ADLocation` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ADLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ad_location: Option<i64>,
}
impl fmt::Debug for BatchSendEmailRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BatchSendEmailRequest")
            .finish_non_exhaustive()
    }
}
