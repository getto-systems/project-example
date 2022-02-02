use actix_web::{HttpRequest, HttpResponse};

use crate::z_lib::remote::response::actix_web::RespondTo;

use crate::auth::ticket::validate::method::ValidateAuthMetadataEvent;

impl RespondTo for ValidateAuthMetadataEvent {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success => HttpResponse::Accepted().finish(),
            Self::MetadataError(err) => err.respond_to(request),
            Self::DecodeError(err) => err.respond_to(request),
        }
    }
}
