use actix_web::HttpResponse;

use crate::common::api::response::x_actix_web::ProxyResponder;

use crate::common::api::service::data::{
    ServiceAuthorizeError, ServiceConnectError, ServiceMetadataError,
};

impl ProxyResponder for ServiceAuthorizeError {
    fn respond_to(self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}

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
