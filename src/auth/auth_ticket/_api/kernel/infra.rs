use crate::{
    auth::auth_ticket::{
        _api::kernel::data::{AuthTokenResponse, AuthTokenMessage},
        _common::kernel::data::{AuthNonce, AuthToken},
    },
    z_details::_api::request::data::HeaderError,
};

pub trait AuthNonceHeader {
    fn nonce(&self) -> Result<Option<AuthNonce>, HeaderError>;
}
pub trait AuthTokenHeader {
    fn token(&self) -> Result<Option<AuthToken>, HeaderError>;
}

pub trait AuthTokenResponseBuilder {
    fn build(&self, message: AuthTokenMessage) -> AuthTokenResponse;
}
