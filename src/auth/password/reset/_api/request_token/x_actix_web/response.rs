use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::event::RequestResetTokenEvent;

use crate::auth::password::reset::_api::request_token::data::{
    EncodeResetTokenError, NotifyResetTokenError, RequestResetTokenResponse,
};

impl RespondTo for RequestResetTokenEvent {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::TokenExpiresCalculated(_) => HttpResponse::Accepted().finish(),
            Self::TokenNotified(_) => HttpResponse::Accepted().finish(),
            Self::Success(response) => response.respond_to(request),
            Self::InvalidReset(response) => response.respond_to(request),
            Self::NonceError(err) => err.respond_to(request),
            Self::RepositoryError(err) => err.respond_to(request),
            Self::MessageError(err) => err.respond_to(request),
            Self::EncodeError(err) => err.respond_to(request),
            Self::NotifyError(err) => err.respond_to(request),
            Self::ValidateLoginIdError(err) => err.respond_to(request),
        }
    }
}

impl RespondTo for RequestResetTokenResponse {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(message) => HttpResponse::Ok().body(message),
            Self::DestinationNotFound(message) => HttpResponse::Ok().body(message),
            Self::UserNotFound(message) => HttpResponse::Ok().body(message),
        }
    }
}

impl RespondTo for EncodeResetTokenError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::InfraError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

impl RespondTo for NotifyResetTokenError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::InfraError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}
