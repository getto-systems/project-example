use actix_web::HttpRequest;

use crate::auth::auth_ticket::_api::kernel::x_actix_web::header::HEADER_NONCE;

use crate::z_details::_api::request::helper::header;

use super::AuthNonceHeader;

use super::super::data::AuthNonceValue;
use crate::z_details::_api::request::data::HeaderError;

pub struct ActixWebAuthNonceHeader {
    request: HttpRequest,
}

impl ActixWebAuthNonceHeader {
    pub const fn new(request: HttpRequest) -> Self {
        Self { request }
    }
}

impl AuthNonceHeader for ActixWebAuthNonceHeader {
    fn nonce(&self) -> Result<AuthNonceValue, HeaderError> {
        header(&self.request, HEADER_NONCE).map(AuthNonceValue::new)
    }
}
