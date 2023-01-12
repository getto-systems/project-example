use crate::{
    auth::{
        kernel::data::{ExpansionLimitDateTime, ExpireDateTime, ExpireDuration},
        ticket::{
            encode::data::EncodeAuthTokenError,
            kernel::data::{AuthTicket, AuthenticateToken, AuthorizeToken, CdnToken},
        },
    },
    common::api::repository::data::RepositoryError,
};

#[async_trait::async_trait]
pub trait EncodeAuthTicketRepository {
    async fn lookup_expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError>;
}

pub trait AuthenticateTokenEncoder {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<(AuthenticateToken, ExpireDateTime), EncodeAuthTokenError>;
}

pub trait AuthorizeTokenEncoder {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<(AuthorizeToken, ExpireDateTime), EncodeAuthTokenError>;
}

pub trait CdnTokenEncoder {
    fn encode(
        &self,
        expires: ExpireDateTime,
    ) -> Result<(CdnToken, ExpireDateTime), EncodeAuthTokenError>;
}

pub struct EncodeAuthTokenConfig {
    pub authenticate_expires: ExpireDuration,
    pub authorize_expires: ExpireDuration,
    pub cdn_expires: ExpireDuration,
}
