use actix_web::{HttpRequest, HttpResponse};

use crate::auth::login_id::_api::data::ValidateLoginIdError;

impl ValidateLoginIdError {
    pub fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Empty => HttpResponse::BadRequest().finish(),
            Self::TooLong => HttpResponse::BadRequest().finish(),
        }
    }
}
