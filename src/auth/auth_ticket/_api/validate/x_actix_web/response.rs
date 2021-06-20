use actix_web::{HttpRequest, HttpResponse};

use super::super::super::kernel::x_actix_web::response::unauthorized;

use super::super::event::ValidateAuthTokenEvent;

use super::super::data::{DecodeAuthTokenError, ValidateAuthTokenError};

impl ValidateAuthTokenEvent {
    pub fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(_) => HttpResponse::Accepted().finish(),
            Self::NonceError(err) => err.respond_to(request),
            Self::TokenError(err) => err.respond_to(request),
        }
    }
}

impl ValidateAuthTokenError {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::HeaderError(err) => err.respond_to(request),
            Self::DecodeError(err) => err.respond_to(request),
            Self::RepositoryError(err) => err.respond_to(request),
            Self::PermissionError(err) => err.respond_to(request),
        }
    }
}

impl DecodeAuthTokenError {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        unauthorized(request)
    }
}
