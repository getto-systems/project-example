use actix_web::{HttpRequest, HttpResponse};

use super::super::action::AuthenticatePasswordState;

impl AuthenticatePasswordState {
    pub fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Authenticate(event) => event.respond_to(request),
            Self::Issue(event) => event.respond_to(request),
            Self::Encode(event) => event.respond_to(request),
        }
    }
}
