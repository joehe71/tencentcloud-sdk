// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `AddressUnsubscribeConfigData` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AddressUnsubscribeConfigData {
    /// Official `Address` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Official `UnsubscribeConfig` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "UnsubscribeConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribe_config: Option<String>,
    /// Official `Status` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}
impl fmt::Debug for AddressUnsubscribeConfigData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddressUnsubscribeConfigData")
            .finish_non_exhaustive()
    }
}
