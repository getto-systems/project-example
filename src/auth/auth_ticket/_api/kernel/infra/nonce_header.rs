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

#[cfg(test)]
pub mod test {
    use super::super::AuthNonceHeader;

    use super::super::super::data::AuthNonceValue;
    use crate::z_details::_api::request::data::HeaderError;

    pub enum StaticAuthNonceHeader {
        Valid(AuthNonceValue),
        NotFound,
    }

    impl AuthNonceHeader for StaticAuthNonceHeader {
        fn nonce(&self) -> Result<AuthNonceValue, HeaderError> {
            match self {
                Self::NotFound => Err(HeaderError::NotFound),
                Self::Valid(nonce) => Ok(nonce.clone()),
            }
        }
    }
}
