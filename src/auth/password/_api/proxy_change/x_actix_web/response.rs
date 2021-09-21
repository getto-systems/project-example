use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use crate::auth::password::_api::proxy_change::data::ChangePasswordProxyMessage;

impl RespondTo for ChangePasswordProxyMessage {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(message) => HttpResponse::Ok().body(message),
            Self::InvalidPassword(message) => HttpResponse::Ok().body(message),
        }
    }
}
