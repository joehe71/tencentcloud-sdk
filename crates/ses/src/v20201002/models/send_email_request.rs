// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use super::{Attachment, Simple, Template};
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `SendEmailRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SendEmailRequest {
    /// Official `FromEmailAddress` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "FromEmailAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_email_address: Option<String>,
    /// Official `Subject` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Official `Destination` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "Destination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub destination: Option<Vec<String>>,
    /// Official `ReplyToAddresses` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ReplyToAddresses",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_to_addresses: Option<String>,
    /// Official `Cc` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Cc", default, skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    /// Official `Bcc` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Bcc", default, skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
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
    /// Official `Unsubscribe` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "Unsubscribe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribe: Option<String>,
    /// Official `TriggerType` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TriggerType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_type: Option<i64>,
    /// Official `SmtpMessageId` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SmtpMessageId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub smtp_message_id: Option<String>,
    /// Official `SmtpHeaders` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SmtpHeaders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub smtp_headers: Option<String>,
    /// Official `HeaderFrom` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "HeaderFrom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub header_from: Option<String>,
}
impl fmt::Debug for SendEmailRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SendEmailRequest").finish_non_exhaustive()
    }
}
