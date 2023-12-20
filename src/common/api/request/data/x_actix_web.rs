use actix_web::HttpRequest;
use uuid::Uuid;

use crate::common::api::request::data::RequestInfo;

impl RequestInfo {
    pub fn from_request(request: &HttpRequest) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            path: request.path().to_owned(),
            method: request.method().to_string(),
        }
    }
}
