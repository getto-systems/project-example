use actix_web::HttpResponse;

use crate::z_lib::response::actix_web::ProxyResponder;

use crate::z_lib::repository::data::RepositoryError;

impl ProxyResponder for RepositoryError {
    fn respond_to(self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}
