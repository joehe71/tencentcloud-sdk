// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `Volume` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Volume {
    /// Official `SendDate` field; omitted when unset and accepts JSON null.
    #[serde(rename = "SendDate", default, skip_serializing_if = "Option::is_none")]
    pub send_date: Option<String>,
    /// Official `RequestCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "RequestCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_count: Option<i64>,
    /// Official `AcceptedCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "AcceptedCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub accepted_count: Option<i64>,
    /// Official `DeliveredCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "DeliveredCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivered_count: Option<i64>,
    /// Official `OpenedCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "OpenedCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub opened_count: Option<i64>,
    /// Official `ClickedCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "ClickedCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clicked_count: Option<i64>,
    /// Official `BounceCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "BounceCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bounce_count: Option<i64>,
    /// Official `UnsubscribeCount` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "UnsubscribeCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribe_count: Option<i64>,
}
impl fmt::Debug for Volume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Volume").finish_non_exhaustive()
    }
}
