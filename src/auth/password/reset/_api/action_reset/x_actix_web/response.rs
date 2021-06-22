use actix_web::{HttpRequest, HttpResponse};

use super::super::action::ResetPasswordState;

impl ResetPasswordState {
    pub fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Reset(event) => event.respond_to(request),
            Self::Issue(event) => event.respond_to(request),
            Self::Encode(event) => event.respond_to(request),
        }
    }
}
