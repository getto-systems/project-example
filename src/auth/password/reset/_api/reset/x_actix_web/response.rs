use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::event::ResetPasswordEvent;

use crate::auth::password::reset::_api::reset::data::ResetPasswordMessage;

impl RespondTo for ResetPasswordEvent {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Result(message) => message.respond_to(request),
            Self::HeaderError(err) => err.respond_to(request),
            Self::ServiceError(err) => err.respond_to(request),
            Self::MessageError(err) => err.respond_to(request),
        }
    }
}

impl RespondTo for ResetPasswordMessage {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(message) => message.respond_to(request),
            Self::InvalidReset(message) => HttpResponse::Ok().body(message),
            Self::AlreadyReset(message) => HttpResponse::Ok().body(message),
        }
    }
}
