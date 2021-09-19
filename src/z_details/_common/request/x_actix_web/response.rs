use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::{request::data::MetadataError, response::actix_web::RespondTo};

impl RespondTo for MetadataError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        HttpResponse::BadRequest().finish()
    }
}
