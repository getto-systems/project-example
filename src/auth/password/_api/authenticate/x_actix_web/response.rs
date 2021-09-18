use actix_web::{HttpRequest, HttpResponse};

use crate::{
    auth::password::_api::authenticate::data::AuthenticatePasswordMessage,
    z_details::_common::response::actix_web::RespondTo,
};

use super::super::event::AuthenticatePasswordEvent;

impl RespondTo for AuthenticatePasswordEvent {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Result(message) => message.respond_to(request),
            Self::MetadataError(err) => err.respond_to(request),
            Self::ServiceError(err) => err.respond_to(request),
            Self::MessageError(err) => err.respond_to(request),
        }
    }
}

impl RespondTo for AuthenticatePasswordMessage {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(message) => message.respond_to(request),
            Self::InvalidPassword(message) => HttpResponse::Ok().body(message),
        }
    }
}
