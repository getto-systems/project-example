use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::action::RequestResetTokenState;

impl RespondTo for RequestResetTokenState {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::RequestToken(event) => event.respond_to(request),
            Self::MessageError(err) => err.respond_to(request),
        }
    }
}
