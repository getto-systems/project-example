use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use crate::example::remote::service::data::ExampleServiceError;

impl RespondTo for ExampleServiceError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Internal(_) => HttpResponse::InternalServerError().finish(),
            Self::Cancelled(_) => HttpResponse::Accepted().finish(),
            Self::InfraError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}
