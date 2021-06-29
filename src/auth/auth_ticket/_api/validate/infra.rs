use crate::auth::auth_ticket::_api::kernel::infra::{
    AuthClock, AuthTicketRepository, CheckAuthNonceInfra,
};

use crate::auth::{
    auth_ticket::_api::{
        kernel::data::{AuthTicket, AuthTokenValue},
        validate::data::DecodeAuthTokenError,
    },
    auth_user::_api::kernel::data::RequireAuthRoles,
};
use crate::z_details::_api::request::data::HeaderError;

pub trait ValidateAuthTokenInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type Clock: AuthClock;
    type TokenHeader: AuthTokenHeader;
    type TicketRepository: AuthTicketRepository;
    type TokenDecoder: AuthTokenDecoder;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn config(&self) -> &ValidateAuthTokenConfig;
    fn clock(&self) -> &Self::Clock;
    fn token_header(&self) -> &Self::TokenHeader;
    fn ticket_repository(&self) -> &Self::TicketRepository;
    fn token_validator(&self) -> &Self::TokenDecoder;
}

pub struct ValidateAuthTokenConfig {
    pub require_roles: RequireAuthRoles,
}

pub trait AuthTokenHeader {
    fn token(&self) -> Result<AuthTokenValue, HeaderError>;
}

pub trait AuthTokenDecoder {
    fn decode(&self, token: &AuthTokenValue) -> Result<AuthTicket, DecodeAuthTokenError>;
}
