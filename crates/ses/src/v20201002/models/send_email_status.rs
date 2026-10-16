// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `SendEmailStatus` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SendEmailStatus {
    /// Official `MessageId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "MessageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Official `ToEmailAddress` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ToEmailAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub to_email_address: Option<String>,
    /// Official `FromEmailAddress` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "FromEmailAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_email_address: Option<String>,
    /// Official `SendStatus` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SendStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub send_status: Option<i64>,
    /// Official `DeliverStatus` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "DeliverStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deliver_status: Option<i64>,
    /// Official `DeliverMessage` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "DeliverMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deliver_message: Option<String>,
    /// Official `RequestTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "RequestTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_time: Option<i64>,
    /// Official `DeliverTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "DeliverTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deliver_time: Option<i64>,
    /// Official `UserOpened` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "UserOpened",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_opened: Option<bool>,
    /// Official `UserClicked` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "UserClicked",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_clicked: Option<bool>,
    /// Official `UserUnsubscribed` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "UserUnsubscribed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_unsubscribed: Option<bool>,
    /// Official `UserComplainted` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "UserComplainted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_complainted: Option<bool>,
    /// Official `UserComplained` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "UserComplained",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_complained: Option<bool>,
}
impl fmt::Debug for SendEmailStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SendEmailStatus").finish_non_exhaustive()
    }
}
