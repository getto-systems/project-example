use actix_web::HttpResponse;

use crate::common::api::{request::data::MetadataError, response::x_actix_web::ProxyResponder};

impl ProxyResponder for MetadataError {
    fn respond_to(self) -> HttpResponse {
        HttpResponse::BadRequest().finish()
    }
}
