use actix_web::HttpResponse;

use crate::z_lib::api::response::actix_web::ProxyResponder;

use crate::z_lib::api::repository::data::RepositoryError;

impl ProxyResponder for RepositoryError {
    fn respond_to(self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}
