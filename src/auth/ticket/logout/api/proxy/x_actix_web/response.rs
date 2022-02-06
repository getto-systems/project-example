use actix_web::HttpResponse;

use crate::z_lib::api::response::actix_web::ProxyResponder;

use crate::auth::ticket::logout::y_protobuf::service::LogoutResponsePb;

impl ProxyResponder for LogoutResponsePb {
    fn respond_to(self) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
