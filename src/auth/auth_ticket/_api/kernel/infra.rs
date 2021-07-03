use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::auth::auth_ticket::_api::kernel::data::{
    AuthDateTime, AuthNonceValue, AuthTicket, AuthTicketExtract, ExpansionLimitDateTime,
    ExpireDateTime, ExpireDuration,
};
use crate::z_details::_api::{
    repository::data::{RegisterResult, RepositoryError},
    request::data::HeaderError,
};

pub trait AuthTicketInfra {
    type Clock: AuthClock;
    type TicketRepository: AuthTicketRepository;

    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
}

pub trait CheckAuthNonceInfra {
    type Clock: AuthClock;
    type NonceHeader: AuthNonceHeader;
    type NonceRepository: AuthNonceRepository;

    fn clock(&self) -> &Self::Clock;
    fn nonce_header(&self) -> &Self::NonceHeader;
    fn nonce_repository(&self) -> &Self::NonceRepository;
    fn config(&self) -> &AuthNonceConfig;
}

pub trait AuthClock {
    fn now(&self) -> AuthDateTime;
}

pub trait AuthNonceHeader {
    fn nonce(&self) -> Result<AuthNonceValue, HeaderError>;
}

pub trait AuthTicketRepository {
    fn register(
        &self,
        ticket: AuthTicket,
        limit: ExpansionLimitDateTime,
        registered_at: AuthDateTime,
    ) -> Result<(), RepositoryError>;

    fn discard(
        &self,
        auth_ticket: AuthTicket,
        discard_at: AuthDateTime,
    ) -> Result<(), RepositoryError>;

    fn expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError>;
}

#[async_trait::async_trait]
pub trait AuthNonceRepository {
    async fn put(
        &self,
        nonce: AuthNonceEntry,
        registered_at: &AuthDateTime,
    ) -> Result<RegisterResult<()>, RepositoryError>;
}

pub struct AuthNonceEntry {
    nonce: AuthNonceValue,
    expires: Option<ExpireDateTime>,
}

impl AuthNonceEntry {
    pub const fn new(nonce: AuthNonceValue, expires: ExpireDateTime) -> Self {
        Self {
            nonce,
            expires: Some(expires),
        }
    }

    #[cfg(test)]
    pub fn nonce(&self) -> &AuthNonceValue {
        &self.nonce
    }

    pub fn extract(self) -> AuthNonceEntryExtract {
        AuthNonceEntryExtract {
            nonce: self.nonce.extract(),
            expires: self.expires.map(|expires| expires.timestamp()),
        }
    }
}

#[derive(Clone)]
pub struct AuthNonceEntryExtract {
    pub nonce: String,
    pub expires: Option<i64>,
}

impl From<AuthNonceEntryExtract> for AuthNonceEntry {
    fn from(src: AuthNonceEntryExtract) -> Self {
        Self {
            nonce: AuthNonceValue::new(src.nonce),
            expires: src.expires.map(|expires| ExpireDateTime::restore(expires)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthJwtClaims {
    aud: String,
    exp: i64,
    ticket_id: String,
    user_id: String,
    granted_roles: HashSet<String>,
}

pub const AUTH_JWT_AUDIENCE_TICKET: &'static str = "ticket";
pub const AUTH_JWT_AUDIENCE_API: &'static str = "api";

impl AuthJwtClaims {
    pub fn from_ticket(ticket: AuthTicket, aud: String, expires: ExpireDateTime) -> Self {
        let ticket = ticket.extract();
        Self {
            aud,
            exp: expires.timestamp(),
            ticket_id: ticket.ticket_id,
            user_id: ticket.user_id,
            granted_roles: ticket.granted_roles,
        }
    }
}

impl Into<AuthTicket> for AuthJwtClaims {
    fn into(self) -> AuthTicket {
        AuthTicketExtract {
            ticket_id: self.ticket_id,
            user_id: self.user_id,
            granted_roles: self.granted_roles,
        }
        .into()
    }
}

pub struct AuthNonceConfig {
    pub nonce_expires: ExpireDuration,
}
