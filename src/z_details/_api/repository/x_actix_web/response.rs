use actix_web::{HttpRequest, HttpResponse};

use super::super::data::RepositoryError;

impl RepositoryError {
    pub fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}
