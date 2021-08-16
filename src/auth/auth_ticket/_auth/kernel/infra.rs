use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    auth::auth_ticket::{
        _auth::kernel::data::{AuthTicket, AuthTicketExtract},
        _common::kernel::data::{
            AuthDateTime, AuthNonceValue, ExpansionLimitDateTime, ExpireDateTime, ExpireDuration,
        },
    },
    z_details::{
        _auth::request::data::MetadataError,
        _common::repository::data::{RegisterResult, RepositoryError},
    },
};

pub trait CheckAuthNonceInfra {
    type Clock: AuthClock;
    type NonceMetadata: AuthNonceMetadata;
    type NonceRepository: AuthNonceRepository;

    fn clock(&self) -> &Self::Clock;
    fn nonce_metadata(&self) -> &Self::NonceMetadata;
    fn nonce_repository(&self) -> &Self::NonceRepository;
    fn config(&self) -> &AuthNonceConfig;
}

pub trait AuthClock: Send + Sync {
    fn now(&self) -> AuthDateTime;
}

pub trait AuthNonceMetadata {
    fn nonce(&self) -> Result<AuthNonceValue, MetadataError>;
}

#[async_trait::async_trait]
pub trait AuthTicketRepository {
    async fn issue(
        &self,
        ticket: AuthTicket,
        limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) -> Result<(), RepositoryError>;

    async fn discard(
        &self,
        auth_ticket: AuthTicket,
        discard_at: AuthDateTime,
    ) -> Result<(), RepositoryError>;

    async fn expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError>;
}

#[async_trait::async_trait]
pub trait AuthNonceRepository {
    async fn put(
        &self,
        nonce: AuthNonceEntry,
        registered_at: AuthDateTime,
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
            expires: self.expires.map(|expires| expires.extract()),
        }
    }
}

#[derive(Clone)]
pub struct AuthNonceEntryExtract {
    pub nonce: String,
    pub expires: Option<DateTime<Utc>>,
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
    pub fn from_ticket(ticket: AuthTicket, aud: String, expires: ExpireDateTime) -> (Self, i64) {
        let ticket = ticket.extract();
        let exp = expires.extract().timestamp();
        (
            Self {
                aud,
                exp,
                ticket_id: ticket.ticket_id,
                user_id: ticket.user_id,
                granted_roles: ticket.granted_roles,
            },
            exp,
        )
    }
}

impl Into<AuthTicket> for AuthJwtClaims {
    fn into(self) -> AuthTicket {
        AuthTicketExtract {
            ticket_id: self.ticket_id,
            user_id: self.user_id,
            granted_roles: self.granted_roles,
        }
        .restore()
    }
}

pub struct AuthNonceConfig {
    pub nonce_expires: ExpireDuration,
}
