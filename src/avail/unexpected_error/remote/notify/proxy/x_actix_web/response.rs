use actix_web::{HttpRequest, HttpResponse};

use crate::z_lib::remote::response::actix_web::RespondTo;

use crate::avail::unexpected_error::remote::notify::proxy::data::NotifyUnexpectedErrorProxyMessage;

impl RespondTo for NotifyUnexpectedErrorProxyMessage {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
