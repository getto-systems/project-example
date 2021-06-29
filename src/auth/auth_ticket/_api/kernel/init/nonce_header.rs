use actix_web::HttpRequest;

use crate::auth::auth_ticket::_api::kernel::x_actix_web::header::HEADER_NONCE;

use crate::z_details::_api::request::helper::header;

use crate::auth::auth_ticket::_api::kernel::infra::AuthNonceHeader;

use crate::auth::auth_ticket::_api::kernel::data::AuthNonceValue;
use crate::z_details::_api::request::data::HeaderError;

pub struct ActixWebAuthNonceHeader<'a> {
    request: &'a HttpRequest,
}

impl<'a> ActixWebAuthNonceHeader<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self { request }
    }
}

impl<'a> AuthNonceHeader for ActixWebAuthNonceHeader<'a> {
    fn nonce(&self) -> Result<AuthNonceValue, HeaderError> {
        header(self.request, HEADER_NONCE).map(AuthNonceValue::new)
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::kernel::infra::AuthNonceHeader;
    
    use crate::auth::auth_ticket::_api::kernel::data::AuthNonceValue;
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
