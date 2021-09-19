use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::event::GetOutlineMenuBadgeEvent;

impl RespondTo for GetOutlineMenuBadgeEvent {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Authorized(_) => HttpResponse::Accepted().finish(),
            Self::Success(menu) => HttpResponse::Ok().body(menu),
            Self::ValidateApiTokenError(err) => err.respond_to(request),
            Self::ServiceError(err) => err.respond_to(request),
            Self::MetadataError(err) => err.respond_to(request),
            Self::MessageError(err) => err.respond_to(request),
        }
    }
}
