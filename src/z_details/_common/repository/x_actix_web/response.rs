use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use crate::z_details::_common::repository::data::RepositoryError;

impl RespondTo for RepositoryError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}
