use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::action::ChangePasswordState;

impl RespondTo for ChangePasswordState {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Change(event) => event.respond_to(request),
            Self::MessageError(err) => err.respond_to(request),
        }
    }
}
