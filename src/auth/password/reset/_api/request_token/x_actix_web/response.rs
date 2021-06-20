use actix_web::{HttpRequest, HttpResponse};

use super::super::event::RequestResetTokenEvent;

use crate::auth::password::reset::_api::request_token::data::{EncodeResetTokenError, NotifyResetTokenError, RequestResetTokenResponse};

impl RequestResetTokenEvent {
    pub fn respond_to(self, request: &HttpRequest) -> HttpResponse {
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

impl RequestResetTokenResponse {
    pub fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        HttpResponse::Ok().body(self.message)
    }
}

impl EncodeResetTokenError {
    pub fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::InfraError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

impl NotifyResetTokenError {
    pub fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::InfraError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}
