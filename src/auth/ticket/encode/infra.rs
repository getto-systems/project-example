use std::collections::HashMap;

use crate::{
    auth::ticket::{
        encode::data::EncodeAuthTokenError,
        remote::kernel::data::{
            AuthTicket, AuthTokenExtract, CloudfrontTokenKind, ExpansionLimitDateTime,
            ExpireDateTime,
        },
    },
    z_lib::remote::repository::data::RepositoryError,
};

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
