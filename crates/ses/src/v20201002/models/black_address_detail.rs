// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `BlackAddressDetail` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlackAddressDetail {
    /// Official `Id` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Official `Email` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Official `CreateTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "CreateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<String>,
    /// Official `ExpireDate` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ExpireDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expire_date: Option<String>,
    /// Official `Status` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}
impl fmt::Debug for BlackAddressDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlackAddressDetail").finish_non_exhaustive()
    }
}
