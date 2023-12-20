use actix_web::HttpResponse;

use crate::common::api::response::x_actix_web::ProxyResponder;

use crate::common::proxy::data::{CoreProxyCallError, CoreProxyError, ProxyResponseBody};

impl ProxyResponder for CoreProxyError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::AuthorizeWithTokenError(err) => err.respond_to(),
            Self::CoreProxyCallError(err) => err.respond_to(),
        }
    }
}

impl ProxyResponder for ProxyResponseBody {
    fn respond_to(self) -> HttpResponse {
        HttpResponse::Ok().body(self.extract())
    }
}

impl ProxyResponder for CoreProxyCallError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::PermissionDenied(_) => HttpResponse::Forbidden().finish(),
            Self::InfraError(_) => HttpResponse::InternalServerError().finish(),
            Self::CheckAuthorizeTokenError(err) => err.respond_to(),
            Self::ValidateAuthorizeTokenError(err) => err.respond_to(),
            Self::ServiceConnectError(err) => err.respond_to(),
            Self::ServiceMetadataError(err) => err.respond_to(),
            Self::ServiceAuthorizeError(err) => err.respond_to(),
            Self::MessageError(err) => err.respond_to(),
        }
    }
}
