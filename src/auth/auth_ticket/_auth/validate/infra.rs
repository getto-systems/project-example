use crate::auth::auth_ticket::_auth::kernel::infra::CheckAuthNonceInfra;

use crate::auth::{
    auth_ticket::_auth::{
        kernel::data::{AuthTicket, AuthTokenValue},
        validate::data::DecodeAuthTokenError,
    },
    auth_user::_common::kernel::data::RequireAuthRoles,
};
use crate::z_details::_auth::request::data::MetadataError;

pub trait ValidateAuthTokenInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type TokenMetadata: AuthTokenMetadata;
    type TokenDecoder: AuthTokenDecoder;

    fn extract(
        self,
    ) -> (
        Self::CheckNonceInfra,
        Self::TokenMetadata,
        Self::TokenDecoder,
        ValidateAuthTokenConfig,
    );
}

pub trait AuthTokenMetadata {
    fn token(&self) -> Result<AuthTokenValue, MetadataError>;
}

pub trait AuthTokenDecoder {
    fn decode(&self, token: &AuthTokenValue) -> Result<AuthTicket, DecodeAuthTokenError>;
}

pub struct ValidateAuthTokenConfig {
    pub require_roles: RequireAuthRoles,
}
