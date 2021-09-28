use actix_web::{HttpRequest, HttpResponse};

use crate::z_lib::remote::response::actix_web::RespondTo;

use crate::auth::user::login_id::remote::data::ValidateLoginIdError;

impl RespondTo for ValidateLoginIdError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Empty => HttpResponse::BadRequest().finish(),
            Self::TooLong => HttpResponse::BadRequest().finish(),
        }
    }
}
