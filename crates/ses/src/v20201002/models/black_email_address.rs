// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `BlackEmailAddress` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlackEmailAddress {
    /// Official `BounceTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "BounceTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bounce_time: Option<String>,
    /// Official `EmailAddress` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EmailAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_address: Option<String>,
    /// Official `IspDesc` field; omitted when unset and accepts JSON null.
    #[serde(rename = "IspDesc", default, skip_serializing_if = "Option::is_none")]
    pub isp_desc: Option<String>,
}
impl fmt::Debug for BlackEmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlackEmailAddress").finish_non_exhaustive()
    }
}
