use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use crate::auth::password::_api::proxy_authenticate::data::AuthenticatePasswordMessage;

impl RespondTo for AuthenticatePasswordMessage {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(message) => message.respond_to(request),
            Self::InvalidPassword(message) => HttpResponse::Ok().body(message),
        }
    }
}
