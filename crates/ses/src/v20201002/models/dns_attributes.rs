// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `DNSAttributes` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DnsAttributes {
    /// Official `Type` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Official `SendDomain` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SendDomain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub send_domain: Option<String>,
    /// Official `ExpectedValue` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ExpectedValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expected_value: Option<String>,
    /// Official `CurrentValue` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "CurrentValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_value: Option<String>,
    /// Official `Status` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}
impl fmt::Debug for DnsAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DnsAttributes").finish_non_exhaustive()
    }
}
