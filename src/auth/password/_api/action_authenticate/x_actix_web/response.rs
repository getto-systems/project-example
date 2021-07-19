use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::action::AuthenticatePasswordState;

impl RespondTo for AuthenticatePasswordState {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Authenticate(event) => event.respond_to(request),
            Self::Issue(event) => event.respond_to(request),
            Self::Encode(event) => event.respond_to(request),
        }
    }
}
