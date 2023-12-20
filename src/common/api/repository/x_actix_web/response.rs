use actix_web::HttpResponse;

use crate::common::api::response::x_actix_web::ProxyResponder;

use crate::common::api::repository::data::RepositoryError;

impl ProxyResponder for RepositoryError {
    fn respond_to(self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}
