use actix_web::HttpResponse;

use crate::z_lib::api::response::actix_web::ProxyResponder;

use crate::auth::ticket::validate::method::ValidateAuthMetadataEvent;

impl ProxyResponder for ValidateAuthMetadataEvent {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::Success => HttpResponse::Accepted().finish(),
            Self::MetadataError(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
        }
    }
}
