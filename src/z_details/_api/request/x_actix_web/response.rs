use actix_web::{HttpRequest, HttpResponse};

use super::super::data::HeaderError;

impl HeaderError {
    pub fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        HttpResponse::BadRequest().finish()
    }
}
