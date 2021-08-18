use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::event::ValidateApiTokenEvent;

impl RespondTo for ValidateApiTokenEvent {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success => HttpResponse::Ok().finish(),
            Self::ServiceError(err) => err.respond_to(request),
            Self::HeaderError(err) => err.respond_to(request),
        }
    }
}
