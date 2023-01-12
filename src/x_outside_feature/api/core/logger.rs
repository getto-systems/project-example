use serde::Serialize;

use crate::x_outside_feature::{core::feature::CoreAppFeature, data::RequestId};

use crate::common::api::logger::init::JsonLogger;

pub struct CoreLogger;

impl CoreLogger {
    pub fn default(
        feature: &CoreAppFeature,
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
