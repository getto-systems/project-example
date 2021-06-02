use actix_web::{HttpRequest, HttpResponse};

use super::super::action::RenewAuthTicketState;

impl RenewAuthTicketState {
    pub fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Validate(event) => event.respond_to(request),
            Self::Encode(event) => event.respond_to(request),
        }
    }
}
