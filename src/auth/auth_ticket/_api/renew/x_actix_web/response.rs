use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::event::RenewAuthTicketEvent;

impl RespondTo for RenewAuthTicketEvent {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(token) => token.respond_to(request),
            Self::HeaderError(err) => err.respond_to(request),
            Self::ServiceError(err) => err.respond_to(request),
            Self::MessageError(err) => err.respond_to(request),
        }
    }
}
