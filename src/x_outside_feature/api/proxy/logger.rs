use actix_web::HttpRequest;
use serde::Serialize;

use crate::x_outside_feature::{data::RequestId, proxy::feature::ProxyAppFeature};

use crate::common::api::logger::init::JsonLogger;

pub struct ProxyLogger;

impl ProxyLogger {
    pub fn default(
        feature: &ProxyAppFeature,
        request: &HttpRequest,
    ) -> (RequestId, JsonLogger<RequestEntry>) {
        let id = RequestId::generate();
        (
            id.clone(),
            JsonLogger::new(feature.log_level, RequestEntry::new(id, request)),
        )
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestEntry {
    id: RequestId,
    path: String,
    method: String,
}

impl RequestEntry {
    fn new(id: RequestId, request: &HttpRequest) -> Self {
        Self {
            id,
            path: request.path().to_string(),
            method: request.method().to_string(),
        }
    }
}
