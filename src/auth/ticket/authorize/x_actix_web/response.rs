use actix_web::HttpResponse;

use crate::common::api::response::actix_web::ProxyResponder;

use crate::auth::ticket::authorize::method::AuthorizeWithTokenEvent;

impl ProxyResponder for AuthorizeWithTokenEvent {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::Success => HttpResponse::Accepted().finish(),
            Self::Invalid(_) => HttpResponse::BadRequest().finish(),
            Self::DecodeError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
        }
    }
}
