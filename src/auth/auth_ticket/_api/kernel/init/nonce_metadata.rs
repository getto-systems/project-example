use actix_web::HttpRequest;

use crate::auth::auth_ticket::_api::kernel::x_actix_web::header::HEADER_NONCE;

use crate::z_details::_api::request::helper::header;

use crate::auth::auth_ticket::_common::kernel::infra::AuthNonceMetadata;

use crate::{
    auth::auth_ticket::_common::kernel::data::AuthNonce,
    z_details::_common::request::data::MetadataError,
};

pub struct ActixWebAuthNonceMetadata<'a> {
    request: &'a HttpRequest,
}

impl<'a> ActixWebAuthNonceMetadata<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self { request }
    }
}

impl<'a> AuthNonceMetadata for ActixWebAuthNonceMetadata<'a> {
    fn nonce(&self) -> Result<Option<AuthNonce>, MetadataError> {
        header(self.request, HEADER_NONCE).map(|value| value.map(AuthNonce::restore))
    }
}
