// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `EmailSender` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct EmailSender {
    /// Official `EmailAddress` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EmailAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_address: Option<String>,
    /// Official `EmailSenderName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EmailSenderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_sender_name: Option<String>,
    /// Official `CreatedTimestamp` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "CreatedTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_timestamp: Option<i64>,
    /// Official `SmtpPwdType` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SmtpPwdType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub smtp_pwd_type: Option<i64>,
}
impl fmt::Debug for EmailSender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EmailSender").finish_non_exhaustive()
    }
}
