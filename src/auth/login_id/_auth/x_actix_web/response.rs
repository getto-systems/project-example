use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use crate::auth::login_id::_auth::data::ValidateLoginIdError;

impl RespondTo for ValidateLoginIdError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Empty => HttpResponse::BadRequest().finish(),
            Self::TooLong => HttpResponse::BadRequest().finish(),
        }
    }
}
