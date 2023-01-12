use actix_web::HttpResponse;

use crate::common::api::response::actix_web::ProxyResponder;

use crate::common::proxy::action::CoreProxyState;

use crate::common::proxy::event::ProxyCallEvent;

use crate::common::proxy::data::{CoreProxyError, ProxyResponseBody};

impl ProxyResponder for CoreProxyState {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::AuthorizeWithToken(event) => event.respond_to(),
            Self::ProxyCall(event) => event.respond_to(),
        }
    }
}

impl<R: ProxyResponder, E: ProxyResponder> ProxyResponder for ProxyCallEvent<R, E> {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::TryToCall(_) => HttpResponse::Accepted().finish(),
            Self::Response(response) => response.respond_to(),
            Self::ServiceError(err) => err.respond_to(),
        }
    }
}

impl ProxyResponder for ProxyResponseBody {
    fn respond_to(self) -> HttpResponse {
        HttpResponse::Ok().body(self.extract())
    }
}

impl ProxyResponder for CoreProxyError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::PermissionDenied(_) => HttpResponse::Forbidden().finish(),
            Self::InfraError(_) => HttpResponse::InternalServerError().finish(),
            Self::ServiceConnectError(err) => err.respond_to(),
            Self::ServiceMetadataError(err) => err.respond_to(),
            Self::MessageError(err) => err.respond_to(),
        }
    }
}
