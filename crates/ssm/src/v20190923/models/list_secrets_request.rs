// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use super::TagFilter;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ListSecretsRequest` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListSecretsRequest {
    /// Official `Offset` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Official `Limit` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Official `OrderType` field; omitted when unset and accepts JSON null.
    #[serde(rename = "OrderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<i64>,
    /// Official `State` field; omitted when unset and accepts JSON null.
    #[serde(rename = "State", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<i64>,
    /// Official `SearchSecretName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SearchSecretName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_secret_name: Option<String>,
    /// Official `TagFilters` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TagFilters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tag_filters: Option<Vec<TagFilter>>,
    /// Official `SecretType` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_type: Option<i64>,
    /// Official `ProductName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ProductName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_name: Option<String>,
    /// Official `EncryptType` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EncryptType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encrypt_type: Option<i64>,
    /// Official `InstanceID` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "InstanceID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_id: Option<String>,
}
impl fmt::Debug for ListSecretsRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListSecretsRequest").finish_non_exhaustive()
    }
}
