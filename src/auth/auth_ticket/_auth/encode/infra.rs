use std::collections::HashMap;

use crate::auth::auth_ticket::_auth::kernel::infra::{AuthClock, AuthTicketRepository};

use crate::auth::auth_ticket::{
    _auth::{
        encode::data::EncodeAuthTokenError,
        kernel::data::{AuthTicket, ExpireDateTime, ExpireDuration},
    },
    _common::kernel::data::{AuthTokenExtract, CloudfrontTokenKind},
};

pub trait EncodeAuthTicketInfra {
    type Clock: AuthClock;
    type TicketRepository: AuthTicketRepository;
    type TicketEncoder: AuthTokenEncoder;
    type ApiEncoder: AuthTokenEncoder;
    type CloudfrontEncoder: CloudfrontTokenEncoder;

    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
    fn ticket_encoder(&self) -> &Self::TicketEncoder;
    fn api_encoder(&self) -> &Self::ApiEncoder;
    fn cloudfront_encoder(&self) -> &Self::CloudfrontEncoder;
    fn config(&self) -> &EncodeAuthTicketConfig;
}

pub trait AuthTokenEncoder {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<AuthTokenExtract, EncodeAuthTokenError>;
}

pub trait CloudfrontTokenEncoder {
    fn encode(
        &self,
        expires: ExpireDateTime,
    ) -> Result<HashMap<CloudfrontTokenKind, AuthTokenExtract>, EncodeAuthTokenError>;
}

pub struct EncodeAuthTicketConfig {
    pub ticket_expires: ExpireDuration,
    pub api_expires: ExpireDuration,
    pub cloudfront_expires: ExpireDuration,
}
