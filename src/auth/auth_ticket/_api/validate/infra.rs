use crate::auth::auth_ticket::_api::kernel::infra::{AuthTicketInfra, CheckAuthNonceInfra};

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
    type TicketInfra: AuthTicketInfra;
    type TokenHeader: AuthTokenHeader;
    type TokenDecoder: AuthTokenDecoder;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn ticket_infra(&self) -> &Self::TicketInfra;
    fn token_header(&self) -> &Self::TokenHeader;
    fn token_validator(&self) -> &Self::TokenDecoder;
    fn config(&self) -> &ValidateAuthTokenConfig;
}

pub trait AuthTokenHeader {
    fn token(&self) -> Result<AuthTokenValue, HeaderError>;
}

pub trait AuthTokenDecoder {
    fn decode(&self, token: &AuthTokenValue) -> Result<AuthTicket, DecodeAuthTokenError>;
}

pub struct ValidateAuthTokenConfig {
    pub require_roles: RequireAuthRoles,
}
