use std::collections::HashMap;

use crate::auth::auth_ticket::_auth::kernel::infra::AuthTicketInfra;

use crate::auth::auth_ticket::{
    _auth::{
        encode::data::EncodeAuthTokenError,
        kernel::data::{AuthTicket, ExpireDateTime, ExpireDuration},
    },
    _common::kernel::data::{AuthTokenExtract, CloudfrontTokenKind},
};

pub trait EncodeAuthTicketInfra {
    type TicketInfra: AuthTicketInfra;
    type TicketEncoder: AuthTokenEncoder;
    type ApiEncoder: AuthTokenEncoder;
    type CloudfrontEncoder: CloudfrontTokenEncoder;

    fn extract(
        self,
    ) -> (
        Self::TicketInfra,
        Self::TicketEncoder,
        Self::ApiEncoder,
        Self::CloudfrontEncoder,
        EncodeAuthTicketConfig,
    );
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
