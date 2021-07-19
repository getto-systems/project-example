use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::action::LogoutState;

impl RespondTo for LogoutState {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Logout(event) => event.respond_to(request),
        }
    }
}
