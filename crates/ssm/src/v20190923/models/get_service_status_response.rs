// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `GetServiceStatusResponse` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetServiceStatusResponse {
    /// Official `ServiceEnabled` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ServiceEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_enabled: Option<bool>,
    /// Official `InvalidType` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "InvalidType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub invalid_type: Option<i64>,
    /// Official `AccessKeyEscrowEnabled` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "AccessKeyEscrowEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_key_escrow_enabled: Option<bool>,
    /// Official `ExpireTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ExpireTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expire_time: Option<String>,
    /// Official `QPSLimit` field; omitted when unset and accepts JSON null.
    #[serde(rename = "QPSLimit", default, skip_serializing_if = "Option::is_none")]
    pub qps_limit: Option<i64>,
    /// Official `SecretLimit` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_limit: Option<i64>,
    /// Official `PayModel` field; omitted when unset and accepts JSON null.
    #[serde(rename = "PayModel", default, skip_serializing_if = "Option::is_none")]
    pub pay_model: Option<String>,
    /// Official `RenewFlag` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RenewFlag", default, skip_serializing_if = "Option::is_none")]
    pub renew_flag: Option<i64>,
    /// Official `ResourceId` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ResourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_id: Option<String>,
    /// Official `TotalCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TotalCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<i64>,
    /// Official `ResourceRegion` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ResourceRegion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_region: Option<i64>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for GetServiceStatusResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GetServiceStatusResponse")
            .finish_non_exhaustive()
    }
}
