// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use super::TagList;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `EmailIdentity` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct EmailIdentity {
    /// Official `IdentityName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "IdentityName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_name: Option<String>,
    /// Official `IdentityType` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "IdentityType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_type: Option<String>,
    /// Official `SendingEnabled` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SendingEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sending_enabled: Option<bool>,
    /// Official `CurrentReputationLevel` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "CurrentReputationLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_reputation_level: Option<i64>,
    /// Official `DailyQuota` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "DailyQuota",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub daily_quota: Option<i64>,
    /// Official `SendIp` field; omitted when unset and accepts JSON null.
    #[serde(rename = "SendIp", default, skip_serializing_if = "Option::is_none")]
    pub send_ip: Option<Vec<String>>,
    /// Official `TagList` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TagList", default, skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<TagList>>,
    /// Official `DKIMOption` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "DKIMOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dkim_option: Option<i64>,
}
impl fmt::Debug for EmailIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EmailIdentity").finish_non_exhaustive()
    }
}
