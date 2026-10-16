// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use super::{ProductPrivilegeUnit, Tag};
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `CreateProductSecretRequest` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateProductSecretRequest {
    /// Official `SecretName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_name: Option<String>,
    /// Official `UserNamePrefix` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "UserNamePrefix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_name_prefix: Option<String>,
    /// Official `ProductName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ProductName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_name: Option<String>,
    /// Official `InstanceID` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "InstanceID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_id: Option<String>,
    /// Official `Domains` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Domains", default, skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// Official `PrivilegesList` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "PrivilegesList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub privileges_list: Option<Vec<ProductPrivilegeUnit>>,
    /// Official `Description` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "Description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Official `KmsKeyId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "KmsKeyId", default, skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// Official `Tags` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// Official `RotationBeginTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "RotationBeginTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rotation_begin_time: Option<String>,
    /// Official `EnableRotation` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EnableRotation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_rotation: Option<bool>,
    /// Official `RotationFrequency` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "RotationFrequency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rotation_frequency: Option<i64>,
    /// Official `KmsHsmClusterId` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "KmsHsmClusterId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kms_hsm_cluster_id: Option<String>,
    /// Official `AccountRemark` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "AccountRemark",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_remark: Option<String>,
    /// Official `AccountType` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "AccountType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_type: Option<String>,
    /// Official `EncryptType` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EncryptType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encrypt_type: Option<i64>,
}
impl fmt::Debug for CreateProductSecretRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CreateProductSecretRequest")
            .finish_non_exhaustive()
    }
}
