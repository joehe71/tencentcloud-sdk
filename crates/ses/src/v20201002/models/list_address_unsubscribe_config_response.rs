// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use super::AddressUnsubscribeConfigData;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ListAddressUnsubscribeConfigResponse` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListAddressUnsubscribeConfigResponse {
    /// Official `AddressUnsubscribeConfigList` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "AddressUnsubscribeConfigList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_unsubscribe_config_list: Option<Vec<AddressUnsubscribeConfigData>>,
    /// Official `Total` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for ListAddressUnsubscribeConfigResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListAddressUnsubscribeConfigResponse")
            .finish_non_exhaustive()
    }
}
