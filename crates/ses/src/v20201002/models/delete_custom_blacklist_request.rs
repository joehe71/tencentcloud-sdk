// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `DeleteCustomBlackListRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteCustomBlacklistRequest {
    /// Official `Emails` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Emails", default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
}
impl fmt::Debug for DeleteCustomBlacklistRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeleteCustomBlacklistRequest")
            .finish_non_exhaustive()
    }
}
