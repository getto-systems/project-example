use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::event::NotifyUnexpectedErrorEvent;

impl RespondTo for NotifyUnexpectedErrorEvent {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Authorized(_) => HttpResponse::Accepted().finish(),
            Self::Notice(_) => HttpResponse::Ok().finish(),
            Self::ValidateApiTokenError(err) => err.respond_to(request),
            Self::MetadataError(err) => err.respond_to(request),
        }
    }
}
