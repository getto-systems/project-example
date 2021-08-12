use crate::{
    auth::auth_ticket::{
        _api::kernel::data::{AuthTokenMessage, AuthTokenMessageEncoded},
        _common::kernel::data::{AuthNonceValue, AuthTokenValue},
    },
    z_details::_api::request::data::HeaderError,
};

pub trait AuthHeaderInfra {
    type NonceHeader: AuthNonceHeader;
    type TokenHeader: AuthTokenHeader;

    fn nonce_header(&self) -> &Self::NonceHeader;
    fn token_header(&self) -> &Self::TokenHeader;
}

pub trait AuthTokenInfra {
    type TokenMessenger: AuthTokenMessenger;

    fn token_messenger(&self) -> &Self::TokenMessenger;
}

pub trait AuthTokenMessenger {
    fn to_message(&self, message: AuthTokenMessageEncoded) -> AuthTokenMessage;
}

pub trait AuthNonceHeader {
    fn nonce(&self) -> Result<AuthNonceValue, HeaderError>;
}

pub trait AuthTokenHeader {
    fn token(&self) -> Result<AuthTokenValue, HeaderError>;
}
