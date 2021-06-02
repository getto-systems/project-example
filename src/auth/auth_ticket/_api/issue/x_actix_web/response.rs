use actix_web::{HttpRequest, HttpResponse};

use super::super::event::IssueAuthTicketEvent;

impl IssueAuthTicketEvent {
    pub fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(_) => HttpResponse::Ok().finish(),
            Self::RepositoryError(err) => err.respond_to(request),
        }
    }
}
