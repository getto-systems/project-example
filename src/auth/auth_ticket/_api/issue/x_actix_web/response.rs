use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::event::IssueAuthTicketEvent;

impl RespondTo for IssueAuthTicketEvent {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::ExpansionLimitCalculated(_) => HttpResponse::Accepted().finish(),
            Self::Success(_) => HttpResponse::Accepted().finish(),
            Self::RepositoryError(err) => err.respond_to(request),
        }
    }
}
