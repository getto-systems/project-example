use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use crate::auth::{
    auth_ticket::_api::kernel::x_actix_web::response::unauthorized,
    password::reset::_api::kernel::data::ValidateResetTokenError,
};

use super::super::event::ResetPasswordEvent;

use crate::auth::password::reset::_api::reset::data::{
    DecodeResetTokenError, ResetPasswordResponse,
};

impl RespondTo for ResetPasswordEvent {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(_) => HttpResponse::Accepted().finish(),
            Self::InvalidReset(response) => response.respond_to(request),
            Self::UserNotFound => HttpResponse::InternalServerError().finish(),
            Self::NonceError(err) => err.respond_to(request),
            Self::RepositoryError(err) => err.respond_to(request),
            Self::PasswordHashError(err) => err.respond_to(request),
            Self::MessageError(err) => err.respond_to(request),
            Self::DecodeError(err) => err.respond_to(request),
            Self::ValidateLoginIdError(err) => err.respond_to(request),
            Self::ValidatePasswordError(err) => err.respond_to(request),
            Self::ValidateResetTokenError(err) => err.respond_to(request),
        }
    }
}

impl ResetPasswordResponse {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::NotFound(message) => HttpResponse::Ok().body(message),
            Self::AlreadyReset(message) => HttpResponse::Ok().body(message),
            Self::Expired(message) => HttpResponse::Ok().body(message),
            Self::InvalidLoginId(message) => HttpResponse::Ok().body(message),
        }
    }
}

impl DecodeResetTokenError {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        unauthorized(request)
    }
}

impl ValidateResetTokenError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Empty => HttpResponse::BadRequest().finish(),
        }
    }
}
