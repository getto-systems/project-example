use std::collections::HashMap;

use crate::auth::ticket::remote::kernel::infra::AuthClock;

use crate::{
    auth::ticket::remote::{
        encode::data::EncodeAuthTokenError,
        kernel::data::{
            AuthTicket, AuthTokenExtract, CloudfrontTokenKind, ExpansionLimitDateTime,
            ExpireDateTime, ExpireDuration,
        },
    },
    z_lib::remote::repository::data::RepositoryError,
};

pub trait EncodeAuthTicketInfra {
    type Clock: AuthClock;
    type TicketRepository: EncodeAuthTicketRepository;
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

#[async_trait::async_trait]
pub trait EncodeAuthTicketRepository {
    async fn find_expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError>;
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
