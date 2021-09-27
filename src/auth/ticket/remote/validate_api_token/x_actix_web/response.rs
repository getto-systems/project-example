use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::data::ValidateApiTokenError;

impl RespondTo for ValidateApiTokenError {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::MetadataError(err) => err.respond_to(request),
            Self::DecodeError(err) => err.respond_to(request),
            Self::ServiceError(err) => err.respond_to(request),
        }
    }
}
