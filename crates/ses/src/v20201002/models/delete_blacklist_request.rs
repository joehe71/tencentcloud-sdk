// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `DeleteBlackListRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteBlacklistRequest {
    /// Official `EmailAddressList` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EmailAddressList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_address_list: Option<Vec<String>>,
}
impl fmt::Debug for DeleteBlacklistRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeleteBlacklistRequest")
            .finish_non_exhaustive()
    }
}
