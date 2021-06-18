use actix_web::{HttpRequest, HttpResponse};

use crate::auth::password::_api::authenticate::data::{AuthenticatePasswordResponse, PasswordMatchError};

use super::super::event::AuthenticatePasswordEvent;

use super::super::data::ValidatePasswordError;

impl AuthenticatePasswordEvent {
    pub fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(_) => HttpResponse::Ok().finish(),
            Self::UserNotFound => HttpResponse::InternalServerError().finish(),
            Self::InvalidPassword(err) => err.respond_to(request),
            Self::NonceError(err) => err.respond_to(request),
            Self::PasswordHashError(err) => err.respond_to(request),
            Self::RepositoryError(err) => err.respond_to(request),
            Self::MessageError(err) => err.respond_to(request),
            Self::ValidateLoginIdError(err) => err.respond_to(request),
            Self::ValidatePasswordError(err) => err.respond_to(request),
        }
    }
}

impl AuthenticatePasswordResponse {
    pub fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        HttpResponse::Ok().body(self.message)
    }
}

impl PasswordMatchError {
    pub fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::InfraError(_) => HttpResponse::InternalServerError().finish()
        }
    }
}

impl ValidatePasswordError {
    pub fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Empty => HttpResponse::BadRequest().finish(),
            Self::TooLong => HttpResponse::BadRequest().finish(),
        }
    }
}
