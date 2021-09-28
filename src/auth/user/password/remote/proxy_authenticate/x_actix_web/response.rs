use actix_web::{HttpRequest, HttpResponse};

use crate::z_lib::remote::response::actix_web::RespondTo;

use crate::auth::user::password::remote::proxy_authenticate::data::AuthenticatePasswordProxyMessage;

impl RespondTo for AuthenticatePasswordProxyMessage {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(message) => message.respond_to(request),
            Self::InvalidPassword(message) => HttpResponse::Ok().body(message),
        }
    }
}
