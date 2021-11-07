use actix_web::{HttpRequest, HttpResponse};

use crate::z_lib::remote::response::actix_web::RespondTo;

use crate::auth::user::password::reset::remote::proxy_request_token::data::RequestResetTokenProxyMessage;

impl RespondTo for RequestResetTokenProxyMessage {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(message) => HttpResponse::Ok().body(message),
            Self::InvalidRequest(message) => HttpResponse::Ok().body(message),
        }
    }
}
