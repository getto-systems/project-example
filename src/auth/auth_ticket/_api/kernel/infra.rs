use crate::{
    auth::auth_ticket::{
        _api::kernel::data::{AuthTokenResponse, AuthTokenMessage},
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
    type ResponseBuilder: AuthTokenResponseBuilder;

    fn response_builder(&self) -> &Self::ResponseBuilder;
}

pub trait AuthTokenResponseBuilder {
    fn build(&self, message: AuthTokenMessage) -> AuthTokenResponse;
}

pub trait AuthNonceHeader {
    fn nonce(&self) -> Result<Option<AuthNonceValue>, HeaderError>;
}

pub trait AuthTokenHeader {
    fn token(&self) -> Result<Option<AuthTokenValue>, HeaderError>;
}
