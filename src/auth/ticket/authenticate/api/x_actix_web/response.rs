use actix_web::HttpResponse;

use crate::common::api::response::actix_web::ProxyResponder;

use crate::auth::ticket::authenticate::method::AuthenticateWithTokenEvent;

impl ProxyResponder for AuthenticateWithTokenEvent {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::Success(_) => HttpResponse::Accepted().finish(),
            Self::Invalid(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
        }
    }
}
