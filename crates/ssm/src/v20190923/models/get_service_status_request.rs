// Protocol metadata reference: Tencent Cloud Java SDK (Apache-2.0).
// Copyright (c) 2017-2025 Tencent. All Rights Reserved.
// Modified: generated Rust models, explicit wire renames, optional fields and redacted Debug.
// See README.md and LICENSE.
use serde::{Deserialize, Serialize};
use std::fmt;
/// The `GetServiceStatusRequest` wire model. Fields follow the official SSM API schema.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetServiceStatusRequest {}
impl fmt::Debug for GetServiceStatusRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GetServiceStatusRequest")
            .finish_non_exhaustive()
    }
}
