use actix_web::HttpResponse;

use crate::common::api::response::actix_web::ProxyResponder;

use crate::common::api::service::data::{ServiceConnectError, ServiceMetadataError};

impl ProxyResponder for ServiceConnectError {
    fn respond_to(self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}

impl ProxyResponder for ServiceMetadataError {
    fn respond_to(self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}
