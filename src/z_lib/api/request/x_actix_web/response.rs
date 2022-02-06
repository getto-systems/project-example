use actix_web::HttpResponse;

use crate::z_lib::api::{request::data::MetadataError, response::actix_web::ProxyResponder};

impl ProxyResponder for MetadataError {
    fn respond_to(self) -> HttpResponse {
        HttpResponse::BadRequest().finish()
    }
}
