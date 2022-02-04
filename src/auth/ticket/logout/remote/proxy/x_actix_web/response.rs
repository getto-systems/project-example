use actix_web::{HttpRequest, HttpResponse};

use crate::z_lib::remote::response::actix_web::RespondTo;

use crate::auth::ticket::y_protobuf::service::LogoutResponsePb;

impl RespondTo for LogoutResponsePb {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
