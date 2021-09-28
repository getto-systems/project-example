use actix_web::{HttpRequest, HttpResponse};

use crate::z_lib::remote::response::actix_web::RespondTo;

use super::super::data::MessageError;

impl RespondTo for MessageError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}
