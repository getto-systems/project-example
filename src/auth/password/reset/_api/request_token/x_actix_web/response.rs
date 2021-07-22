use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::event::RequestResetTokenEvent;

use crate::auth::password::reset::_api::request_token::data::RequestResetTokenResult;

impl RespondTo for RequestResetTokenEvent {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Result(result) => result.respond_to(request),
            Self::HeaderError(err) => err.respond_to(request),
            Self::ServiceError(err) => err.respond_to(request),
            Self::MessageError(err) => err.respond_to(request),
        }
    }
}

impl RespondTo for RequestResetTokenResult {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(message) => HttpResponse::Ok().body(message),
            Self::InvalidRequest(message) => HttpResponse::Ok().body(message),
        }
    }
}
