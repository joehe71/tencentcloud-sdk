// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use super::SendEmailStatus;
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `GetSendEmailStatusResponse` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSendEmailStatusResponse {
    /// Official `EmailStatusList` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "EmailStatusList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub email_status_list: Option<Vec<SendEmailStatus>>,
    /// Official `RequestId` field; omitted when unset and accepts JSON null.
    #[serde(rename = "RequestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl fmt::Debug for GetSendEmailStatusResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GetSendEmailStatusResponse")
            .finish_non_exhaustive()
    }
}
