use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use crate::auth::password::_api::kernel::data::{PasswordHashError, ValidatePasswordError};

impl RespondTo for PasswordHashError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::InfraError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

impl RespondTo for ValidatePasswordError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Empty => HttpResponse::BadRequest().finish(),
            Self::TooLong => HttpResponse::BadRequest().finish(),
        }
    }
}
