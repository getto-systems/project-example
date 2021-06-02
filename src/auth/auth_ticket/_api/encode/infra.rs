pub mod messenger;
pub mod token_encoder;

use super::super::kernel::infra::{AuthClock, AuthTicketRepository};

use crate::auth::auth_user::_api::kernel::data::GrantedAuthRoles;
use super::super::kernel::data::{AuthTicket, ExpireDateTime, ExpireDuration};
use super::data::{AuthTokenEncoded, EncodeAuthTokenError};
use crate::z_details::_api::message::data::MessageError;

pub trait EncodeAuthTicketInfra {
    type Clock: AuthClock;
    type TicketRepository: AuthTicketRepository;
    type TicketEncoder: AuthTokenEncoder;
    type ApiEncoder: AuthTokenEncoder;
    type CdnEncoder: AuthTokenEncoder;
    type Messenger: EncodeMessenger;

    fn config(&self) -> &EncodeAuthTicketConfig;
    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
    fn ticket_encoder(&self) -> &Self::TicketEncoder;
    fn api_encoder(&self) -> &Self::ApiEncoder;
    fn cdn_encoder(&self) -> &Self::CdnEncoder;
    fn messenger(&self) -> &Self::Messenger;
}

pub struct EncodeAuthTicketConfig {
    pub ticket_expires: ExpireDuration,
    pub api_expires: ExpireDuration,
    pub cdn_expires: ExpireDuration,
}

pub trait AuthTokenEncoder {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<Vec<AuthTokenEncoded>, EncodeAuthTokenError>;
}

pub trait EncodeMessenger {
    fn encode(&self, granted_roles: GrantedAuthRoles) -> Result<String, MessageError>;
}
