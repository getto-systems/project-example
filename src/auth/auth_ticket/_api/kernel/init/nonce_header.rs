use actix_web::HttpRequest;

use crate::auth::auth_ticket::_api::kernel::x_actix_web::header::HEADER_NONCE;

use crate::z_details::_api::request::helper::header;

use crate::auth::auth_ticket::_api::kernel::infra::AuthNonceHeader;

use crate::{
    auth::auth_ticket::_common::kernel::data::AuthNonce,
    z_details::_api::request::data::HeaderError,
};

pub struct ActixWebAuthNonceHeader<'a> {
    request: &'a HttpRequest,
}

impl<'a> ActixWebAuthNonceHeader<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self { request }
    }
}

impl<'a> AuthNonceHeader for ActixWebAuthNonceHeader<'a> {
    fn nonce(&self) -> Result<Option<AuthNonce>, HeaderError> {
        header(self.request, HEADER_NONCE).map(|value| value.map(AuthNonce::new))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::kernel::infra::AuthNonceHeader;

    use crate::{
        auth::auth_ticket::_common::kernel::data::AuthNonce,
        z_details::_api::request::data::HeaderError,
    };

    pub enum StaticAuthNonceHeader {
        Valid(AuthNonce),
    }

    impl AuthNonceHeader for StaticAuthNonceHeader {
        fn nonce(&self) -> Result<Option<AuthNonce>, HeaderError> {
            match self {
                Self::Valid(nonce) => Ok(Some(nonce.clone())),
            }
        }
    }
}
