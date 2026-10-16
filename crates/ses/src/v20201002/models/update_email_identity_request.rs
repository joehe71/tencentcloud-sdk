// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `UpdateEmailIdentityRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateEmailIdentityRequest {
    /// Official `EmailIdentity` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EmailIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_identity: Option<String>,
    /// Official `NewAPI` field; omitted when unset and accepts JSON null.
    #[serde(rename = "NewAPI", default, skip_serializing_if = "Option::is_none")]
    pub new_api: Option<bool>,
    /// Official `DKIMOption` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "DKIMOption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dkim_option: Option<i64>,
}
impl fmt::Debug for UpdateEmailIdentityRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UpdateEmailIdentityRequest")
            .finish_non_exhaustive()
    }
}
