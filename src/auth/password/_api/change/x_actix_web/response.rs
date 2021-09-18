use actix_web::{HttpRequest, HttpResponse};

use crate::{
    auth::password::_api::change::data::ChangePasswordResult,
    z_details::_common::response::actix_web::RespondTo,
};

use super::super::event::ChangePasswordEvent;

impl RespondTo for ChangePasswordEvent {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Result(message) => message.respond_to(request),
            Self::MetadataError(err) => err.respond_to(request),
            Self::ServiceError(err) => err.respond_to(request),
            Self::MessageError(err) => err.respond_to(request),
        }
    }
}

impl RespondTo for ChangePasswordResult {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(message) => HttpResponse::Ok().body(message),
            Self::InvalidPassword(message) => HttpResponse::Ok().body(message),
        }
    }
}
