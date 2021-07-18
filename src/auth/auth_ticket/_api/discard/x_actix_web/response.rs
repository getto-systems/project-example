use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::event::DiscardAuthTicketEvent;

impl RespondTo for DiscardAuthTicketEvent {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success => HttpResponse::Ok().finish(),
            Self::RepositoryError(err) => err.respond_to(request),
        }
    }
}
