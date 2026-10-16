// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use super::{DnsAttributes, TagList};
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `GetEmailIdentityResponse` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEmailIdentityResponse {
    /// Official `IdentityType` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "IdentityType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_type: Option<String>,
    /// Official `VerifiedForSendingStatus` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "VerifiedForSendingStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verified_for_sending_status: Option<bool>,
    /// Official `Attributes` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "Attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<Vec<DnsAttributes>>,
    /// Official `DKIMOption` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "DKIMOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dkim_option: Option<i64>,
    /// Official `TagList` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TagList", default, skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<TagList>>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for GetEmailIdentityResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GetEmailIdentityResponse")
            .finish_non_exhaustive()
    }
}
