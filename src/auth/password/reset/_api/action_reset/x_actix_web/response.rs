use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::action::ResetPasswordState;

impl RespondTo for ResetPasswordState {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Reset(event) => event.respond_to(request),
        }
    }
}
