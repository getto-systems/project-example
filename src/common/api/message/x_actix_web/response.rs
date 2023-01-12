use actix_web::HttpResponse;

use crate::common::api::response::actix_web::ProxyResponder;

use super::super::data::MessageError;

impl ProxyResponder for MessageError {
    fn respond_to(self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}
