use actix_web::{HttpRequest, HttpResponse};

use crate::z_lib::remote::response::actix_web::RespondTo;

use crate::auth::user::account::remote::search::proxy::data::SearchAuthUserAccountProxyMessage;

impl RespondTo for SearchAuthUserAccountProxyMessage {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(message) => HttpResponse::Ok().body(message),
        }
    }
}
