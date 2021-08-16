use crate::auth::auth_ticket::_auth::kernel::infra::CheckAuthNonceInfra;

use crate::{
    auth::{
        auth_ticket::{
            _auth::{kernel::data::AuthTicket, validate::data::DecodeAuthTokenError},
            _common::kernel::data::AuthTokenValue,
        },
        auth_user::_auth::kernel::data::RequireAuthRoles,
    },
    z_details::_auth::request::data::MetadataError,
};

pub trait ValidateAuthTokenInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type TokenMetadata: AuthTokenMetadata;
    type TokenDecoder: AuthTokenDecoder;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn token_metadata(&self) -> &Self::TokenMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
    fn config(&self) -> &ValidateAuthTokenConfig;
}

pub trait AuthTokenMetadata {
    fn token(&self) -> Result<Option<AuthTokenValue>, MetadataError>;
}

pub trait AuthTokenDecoder {
    fn decode(&self, token: &AuthTokenValue) -> Result<AuthTicket, DecodeAuthTokenError>;
}

pub struct ValidateAuthTokenConfig {
    pub require_roles: RequireAuthRoles,
}
