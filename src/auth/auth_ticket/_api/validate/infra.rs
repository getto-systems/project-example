pub mod token_header;
pub mod token_validator;

use super::super::kernel::infra::{AuthClock, AuthTicketRepository};

use super::super::kernel::data::{AuthTicket, AuthTokenValue};
use super::data::DecodeAuthTokenError;
use crate::auth::auth_ticket::_api::kernel::infra::CheckAuthNonceInfra;
use crate::auth::auth_user::_api::kernel::data::RequireAuthRoles;
use crate::z_details::_api::request::data::HeaderError;

pub trait ValidateAuthTokenInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type Clock: AuthClock;
    type TokenHeader: AuthTokenHeader;
    type TicketRepository: AuthTicketRepository;
    type TokenValidator: AuthTokenValidator;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn config(&self) -> &ValidateAuthTokenConfig;
    fn clock(&self) -> &Self::Clock;
    fn token_header(&self) -> &Self::TokenHeader;
    fn ticket_repository(&self) -> &Self::TicketRepository;
    fn token_validator(&self) -> &Self::TokenValidator;
}

pub struct ValidateAuthTokenConfig {
    pub require_roles: RequireAuthRoles,
}

pub trait AuthTokenHeader {
    fn token(&self) -> Result<AuthTokenValue, HeaderError>;
}

pub trait AuthTokenValidator {
    fn validate(&self, token: &AuthTokenValue) -> Result<AuthTicket, DecodeAuthTokenError>;
}
