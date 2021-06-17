use actix_web::{HttpRequest, HttpResponse};

use super::super::action::LogoutState;

impl LogoutState {
    pub fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Validate(event) => event.respond_to(request),
            Self::Discard(event) => event.respond_to(request),
        }
    }
}
