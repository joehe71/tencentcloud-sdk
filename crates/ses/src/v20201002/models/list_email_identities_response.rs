// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use super::EmailIdentity;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ListEmailIdentitiesResponse` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListEmailIdentitiesResponse {
    /// Official `EmailIdentities` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EmailIdentities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_identities: Option<Vec<EmailIdentity>>,
    /// Official `MaxReputationLevel` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "MaxReputationLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_reputation_level: Option<i64>,
    /// Official `MaxDailyQuota` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "MaxDailyQuota",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_daily_quota: Option<i64>,
    /// Official `Total` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for ListEmailIdentitiesResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListEmailIdentitiesResponse")
            .finish_non_exhaustive()
    }
}
