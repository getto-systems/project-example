use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::data::{
    AuthDateTime, AuthNonceValue, AuthTicket, AuthTicketExtract, AuthTicketId,
    ExpansionLimitDateTime, ExpireDateTime, ExpireDuration,
};
use crate::z_details::_api::{
    repository::data::{RegisterAttemptResult, RepositoryError},
    request::data::HeaderError,
};

pub trait CheckAuthNonceInfra {
    type Clock: AuthClock;
    type NonceHeader: AuthNonceHeader;
    type NonceRepository: AuthNonceRepository;

    fn config(&self) -> &AuthNonceConfig;
    fn clock(&self) -> &Self::Clock;
    fn nonce_header(&self) -> &Self::NonceHeader;
    fn nonce_repository(&self) -> &Self::NonceRepository;
}

pub struct AuthNonceConfig {
    pub nonce_expires: ExpireDuration,
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
        ticket_id: AuthTicketId,
        limit: ExpansionLimitDateTime,
        registered_at: AuthDateTime,
    ) -> Result<RegisterAttemptResult<AuthTicketId>, RepositoryError>;

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

pub trait AuthTicketIdGenerator {
    fn generate(&self) -> AuthTicketId;
}

pub trait AuthNonceRepository {
    fn get(&self, nonce: &AuthNonceValue) -> Result<Option<AuthNonceEntry>, RepositoryError>;
    fn put(&self, nonce: AuthNonceEntry) -> Result<(), RepositoryError>;
}

#[derive(Clone)]
pub struct AuthNonceEntry {
    nonce: AuthNonceValue,
    expires: ExpireDateTime,
}

impl AuthNonceEntry {
    pub const fn new(nonce: AuthNonceValue, expires: ExpireDateTime) -> Self {
        Self { nonce, expires }
    }

    pub fn into_nonce(self) -> AuthNonceValue {
        self.nonce
    }

    pub fn has_expired(&self, now: &AuthDateTime) -> bool {
        self.expires.has_elapsed(now)
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
