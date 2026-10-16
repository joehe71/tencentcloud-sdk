// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use super::TagList;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `CreateEmailIdentityRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateEmailIdentityRequest {
    /// Official `EmailIdentity` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EmailIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_identity: Option<String>,
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
}
impl fmt::Debug for CreateEmailIdentityRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CreateEmailIdentityRequest")
            .finish_non_exhaustive()
    }
}
