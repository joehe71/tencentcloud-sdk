// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md, licenses/Apache-2.0.txt and scripts/generate-product.py.
use super::TemplateContent;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `CreateEmailTemplateRequest` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateEmailTemplateRequest {
    /// Official `TemplateName` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TemplateName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_name: Option<String>,
    /// Official `TemplateContent` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "TemplateContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_content: Option<TemplateContent>,
}
impl fmt::Debug for CreateEmailTemplateRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CreateEmailTemplateRequest")
            .finish_non_exhaustive()
    }
}
