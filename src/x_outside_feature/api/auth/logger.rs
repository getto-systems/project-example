use serde::Serialize;

use crate::x_outside_feature::{auth::feature::AuthAppFeature, data::RequestId};

use crate::common::api::logger::init::JsonLogger;

pub struct AuthLogger;

impl AuthLogger {
    pub fn default(
        feature: &AuthAppFeature,
        target: &'static str,
        request_id: RequestId,
    ) -> JsonLogger<RequestEntry> {
        JsonLogger::new(feature.log_level, RequestEntry::new(target, request_id))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestEntry {
    id: RequestId,
    target: &'static str,
}

impl RequestEntry {
    fn new(target: &'static str, id: RequestId) -> Self {
        Self { id, target }
    }
}
