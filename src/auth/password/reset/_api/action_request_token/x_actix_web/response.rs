use actix_web::{HttpRequest, HttpResponse};

use super::super::action::RequestResetTokenState;

impl RequestResetTokenState {
    pub fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::RequestToken(event) => event.respond_to(request),
        }
    }
}
