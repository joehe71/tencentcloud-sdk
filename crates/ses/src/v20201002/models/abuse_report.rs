// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `AbuseReport` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AbuseReport {
    /// Official `DeliverTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "DeliverTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deliver_time: Option<String>,
    /// Official `OriginalMailFrom` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "OriginalMailFrom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_mail_from: Option<String>,
    /// Official `OriginalRcptTo` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "OriginalRcptTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_rcpt_to: Option<String>,
    /// Official `FromDomain` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "FromDomain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub from_domain: Option<String>,
    /// Official `ComplainTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ComplainTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub complain_time: Option<String>,
    /// Official `Mta` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Mta", default, skip_serializing_if = "Option::is_none")]
    pub mta: Option<String>,
    /// Official `SourceIp` field; omitted when unset and accepts JSON null.
    #[serde(rename = "SourceIp", default, skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
    /// Official `InsertTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "InsertTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub insert_time: Option<String>,
    /// Official `TemplateId` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TemplateId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_id: Option<String>,
    /// Official `BulkId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "BulkId", default, skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,
    /// Official `MessageId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "MessageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Official `AbuseTime` field; omitted when unset and accepts JSON null.
    #[serde(rename = "AbuseTime", default, skip_serializing_if = "Option::is_none")]
    pub abuse_time: Option<String>,
    /// Official `Subject` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}
impl fmt::Debug for AbuseReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AbuseReport").finish_non_exhaustive()
    }
}
