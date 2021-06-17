use actix_web::{HttpRequest, HttpResponse};

use super::super::event::DiscardAuthTicketEvent;

impl DiscardAuthTicketEvent {
    pub fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success => HttpResponse::Ok().finish(),
            Self::RepositoryError(err) => err.respond_to(request),
        }
    }
}
