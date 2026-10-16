// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use super::SecretAccountInfo;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `DescribeSecretResponse` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DescribeSecretResponse {
    /// Official `SecretName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SecretName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_name: Option<String>,
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
    /// Official `CreateUin` field; omitted when unset and accepts JSON null.
    #[serde(rename = "CreateUin", default, skip_serializing_if = "Option::is_none")]
    pub create_uin: Option<i64>,
    /// Official `Status` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Official `DeleteTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "DeleteTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_time: Option<i64>,
    /// Official `CreateTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "CreateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
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
    /// Official `ResourceID` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ResourceID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_id: Option<String>,
    /// Official `RotationStatus` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "RotationStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rotation_status: Option<bool>,
    /// Official `RotationFrequency` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "RotationFrequency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rotation_frequency: Option<i64>,
    /// Official `ResourceName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ResourceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_name: Option<String>,
    /// Official `ProjectID` field; omitted when unset and accepts JSON null.
    #[serde(rename = "ProjectID", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    /// Official `AssociatedInstanceIDs` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "AssociatedInstanceIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub associated_instance_i_ds: Option<Vec<String>>,
    /// Official `TargetUin` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TargetUin", default, skip_serializing_if = "Option::is_none")]
    pub target_uin: Option<i64>,
    /// Official `AdditionalConfig` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "AdditionalConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_config: Option<String>,
    /// Official `EncryptType` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EncryptType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encrypt_type: Option<i64>,
    /// Official `EncryptSwitching` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EncryptSwitching",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encrypt_switching: Option<bool>,
    /// Official `CreateUinString` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "CreateUinString",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_uin_string: Option<String>,
    /// Official `TargetUinString` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TargetUinString",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_uin_string: Option<String>,
    /// Official `AccountInfoList` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "AccountInfoList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_info_list: Option<Vec<SecretAccountInfo>>,
    /// Official `NextRotationTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "NextRotationTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_rotation_time: Option<String>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for DescribeSecretResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DescribeSecretResponse")
            .finish_non_exhaustive()
    }
}
