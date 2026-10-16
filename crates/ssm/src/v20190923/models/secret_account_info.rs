// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `SecretAccountInfo` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SecretAccountInfo {
    /// Official `AccountName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "AccountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_name: Option<String>,
    /// Official `Host` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Host", default, skip_serializing_if = "Option::is_none")]
    pub host: Option<Vec<String>>,
    /// Official `Version` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Official `RotatedTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "RotatedTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rotated_time: Option<String>,
}
impl fmt::Debug for SecretAccountInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecretAccountInfo").finish_non_exhaustive()
    }
}
