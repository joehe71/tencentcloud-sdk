// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use super::BlackEmailAddress;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ListBlackEmailAddressResponse` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListBlackEmailAddressResponse {
    /// Official `BlackList` field; omitted when unset and accepts JSON null.
    #[serde(rename = "BlackList", default, skip_serializing_if = "Option::is_none")]
    pub blacklist: Option<Vec<BlackEmailAddress>>,
    /// Official `TotalCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TotalCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i64>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for ListBlackEmailAddressResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListBlackEmailAddressResponse")
            .finish_non_exhaustive()
    }
}
