use actix_web::{HttpRequest, HttpResponse};

use crate::z_lib::remote::response::actix_web::RespondTo;

use crate::z_lib::remote::repository::data::RepositoryError;

impl RespondTo for RepositoryError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}
