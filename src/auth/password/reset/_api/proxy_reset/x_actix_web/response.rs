use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use crate::auth::password::reset::_api::proxy_reset::data::ResetPasswordProxyMessage;

impl RespondTo for ResetPasswordProxyMessage {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(message) => message.respond_to(request),
            Self::InvalidReset(message) => HttpResponse::Ok().body(message),
            Self::AlreadyReset(message) => HttpResponse::Ok().body(message),
        }
    }
}
