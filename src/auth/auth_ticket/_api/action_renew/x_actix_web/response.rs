use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::action::RenewAuthTicketState;

impl RespondTo for RenewAuthTicketState {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Renew(event) => event.respond_to(request),
        }
    }
}
