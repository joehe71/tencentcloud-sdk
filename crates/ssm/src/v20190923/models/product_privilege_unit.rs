// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `ProductPrivilegeUnit` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ProductPrivilegeUnit {
    /// Official `PrivilegeName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "PrivilegeName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub privilege_name: Option<String>,
    /// Official `Privileges` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "Privileges",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub privileges: Option<Vec<String>>,
    /// Official `Database` field; omitted when unset and accepts JSON null.
    #[serde(rename = "Database", default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// Official `TableName` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// Official `ColumnName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ColumnName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub column_name: Option<String>,
    /// Official `SchemaName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SchemaName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub schema_name: Option<String>,
    /// Official `SequenceName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "SequenceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sequence_name: Option<String>,
    /// Official `ProcedureName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ProcedureName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub procedure_name: Option<String>,
    /// Official `TypeName` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TypeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    /// Official `FunctionName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "FunctionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub function_name: Option<String>,
    /// Official `ViewName` field; omitted when unset and accepts JSON null.
    #[serde(rename = "ViewName", default, skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
    /// Official `MatviewName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "MatviewName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub matview_name: Option<String>,
}
impl fmt::Debug for ProductPrivilegeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProductPrivilegeUnit")
            .finish_non_exhaustive()
    }
}
