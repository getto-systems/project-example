use actix_web::HttpResponse;

use crate::common::api::response::x_actix_web::ProxyResponder;

use crate::auth::ticket::kernel::data::ValidateAuthorizeTokenError;

impl ProxyResponder for ValidateAuthorizeTokenError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::NotFound => HttpResponse::BadRequest().finish(),
            Self::MetadataError(err) => err.respond_to(),
        }
    }
}
