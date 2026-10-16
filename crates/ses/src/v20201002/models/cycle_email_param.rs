// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See THIRD_PARTY_NOTICES.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `CycleEmailParam` wire model. Fields follow the official SES API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CycleEmailParam {
    /// Official `BeginTime` field; omitted when unset and accepts JSON null.
    #[serde(rename = "BeginTime", default, skip_serializing_if = "Option::is_none")]
    pub begin_time: Option<String>,
    /// Official `IntervalTime` field; omitted when unset and accepts JSON null.
    #[serde(
        rename = "IntervalTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interval_time: Option<i64>,
    /// Official `TermCycle` field; omitted when unset and accepts JSON null.
    #[serde(rename = "TermCycle", default, skip_serializing_if = "Option::is_none")]
    pub term_cycle: Option<i64>,
}
impl fmt::Debug for CycleEmailParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CycleEmailParam").finish_non_exhaustive()
    }
}
