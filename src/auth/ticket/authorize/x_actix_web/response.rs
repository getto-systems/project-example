use actix_web::HttpResponse;

use crate::common::api::response::x_actix_web::ProxyResponder;

use crate::auth::ticket::authorize::data::{AuthorizeWithTokenError, CheckAuthorizeTokenError};

impl ProxyResponder for CheckAuthorizeTokenError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::Invalid(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
        }
    }
}

impl ProxyResponder for AuthorizeWithTokenError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::Invalid(_) => HttpResponse::BadRequest().finish(),
            Self::DecodeError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
        }
    }
}
