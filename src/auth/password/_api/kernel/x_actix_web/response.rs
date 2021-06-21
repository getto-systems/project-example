use actix_web::{HttpRequest, HttpResponse};

use crate::auth::password::_api::kernel::data::{PasswordHashError, ValidatePasswordError};

impl PasswordHashError {
    pub fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::InfraError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

impl ValidatePasswordError {
    pub fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Empty => HttpResponse::BadRequest().finish(),
            Self::TooLong => HttpResponse::BadRequest().finish(),
        }
    }
}
