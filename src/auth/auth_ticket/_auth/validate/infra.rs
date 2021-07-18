use crate::auth::auth_ticket::_auth::kernel::infra::{AuthTicketInfra, CheckAuthNonceInfra};

use crate::auth::{
    auth_ticket::_auth::{
        kernel::data::{AuthTicket, AuthTokenValue},
        validate::data::DecodeAuthTokenError,
    },
    auth_user::_auth::kernel::data::RequireAuthRoles,
};
use crate::z_details::_auth::request::data::MetadataError;

pub trait ValidateAuthTokenInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type TicketInfra: AuthTicketInfra;
    type TokenMetadata: AuthTokenMetadata;
    type TokenDecoder: AuthTokenDecoder;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn ticket_infra(&self) -> &Self::TicketInfra;
    fn token_metadata(&self) -> &Self::TokenMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
    fn config(&self) -> &ValidateAuthTokenConfig;
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
