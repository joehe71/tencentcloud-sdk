// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `CreateEmailAddressRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateEmailAddressRequest {
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
}
impl fmt::Debug for CreateEmailAddressRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CreateEmailAddressRequest")
            .finish_non_exhaustive()
    }
}
