pub mod header;
pub mod token_validator;

use super::super::kernel::infra::{
    AuthClock, AuthNonceConfig, AuthNonceHeader, AuthNonceRepository, AuthTicketRepository,
};

use super::super::kernel::data::{AuthTicket, AuthTokenValue};
use super::data::DecodeAuthTokenError;
use crate::auth::auth_user::_api::kernel::data::RequireAuthRoles;
use crate::z_details::_api::request::data::HeaderError;

pub trait ValidateAuthTokenInfra {
    type Clock: AuthClock;
    type NonceHeader: AuthNonceHeader;
    type TokenHeader: AuthTokenHeader;
    type NonceRepository: AuthNonceRepository;
    type TicketRepository: AuthTicketRepository;
    type TokenValidator: AuthTokenValidator;

    fn config(&self) -> &ValidateConfig;
    fn nonce_config(&self) -> &AuthNonceConfig;
    fn clock(&self) -> &Self::Clock;
    fn nonce_header(&self) -> &Self::NonceHeader;
    fn token_header(&self) -> &Self::TokenHeader;
    fn nonce_repository(&self) -> &Self::NonceRepository;
    fn ticket_repository(&self) -> &Self::TicketRepository;
    fn token_validator(&self) -> &Self::TokenValidator;
}

pub struct ValidateConfig {
    pub require_roles: RequireAuthRoles,
}

pub trait AuthTokenHeader {
    fn token(&self) -> Result<AuthTokenValue, HeaderError>;
}

pub trait AuthTokenValidator {
    fn validate(&self, token: &AuthTokenValue) -> Result<AuthTicket, DecodeAuthTokenError>;
}
