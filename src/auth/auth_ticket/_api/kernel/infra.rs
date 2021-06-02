pub mod clock;
pub mod header;
pub mod nonce_repository;
pub mod ticket_repository;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::data::{
    AuthDateTime, AuthNonceValue, AuthTicket, AuthTicketExtract, AuthTicketId, AuthToken,
    AuthTokenValue, ExpansionLimitDateTime, ExpireDateTime, ExpireDuration,
};
use crate::z_details::_api::{repository::data::RepositoryError, request::data::HeaderError};

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
        id_generator: impl Fn() -> AuthTicketId,
        registered_at: AuthDateTime,
        limit: ExpansionLimitDateTime,
    ) -> Result<AuthTicketId, RepositoryError>;

    fn register_tokens(
        &self,
        ticket: AuthTicket,
        tokens: AuthTicketTokens,
    ) -> Result<(), RepositoryError>;

    fn expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError>;

    fn disable(&self, token: &AuthTokenValue) -> Result<(), RepositoryError>;
}

pub struct AuthTicketTokens(Vec<AuthToken>);

impl AuthTicketTokens {
    pub const fn new(tokens: Vec<AuthToken>) -> Self {
        Self(tokens)
    }

    pub fn extract(self) -> Vec<AuthToken> {
        self.0
    }
}

pub trait AuthNonceRepository {
    fn get(&self, nonce: &AuthNonceValue) -> Result<Option<AuthNonceEntry>, RepositoryError>;
    fn put(&self, nonce: AuthNonceEntry) -> Result<(), RepositoryError>;
}

pub struct AuthNonceEntry {
    nonce: AuthNonceValue,
    expires: ExpireDateTime,
}

impl AuthNonceEntry {
    pub const fn new(nonce: AuthNonceValue, expires: ExpireDateTime) -> Self {
        Self { nonce, expires }
    }

    pub fn has_elapsed(&self, now: AuthDateTime) -> bool {
        self.expires.has_elapsed(now)
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthJwtClaims {
    aud: String,
    exp: i64,
    auth_ticket_id: String,
    user_id: String,
    granted_roles: HashSet<String>,
}

pub const AUTH_JWT_AUDIENCE_TICKET: &'static str = "ticket";
pub const AUTH_JWT_AUDIENCE_API: &'static str = "api";

impl AuthTicket {
    pub fn into_jwt_claims(self, aud: String, expires: ExpireDateTime) -> AuthJwtClaims {
        let auth_ticket = self.extract();
        AuthJwtClaims {
            aud,
            exp: expires.timestamp(),
            auth_ticket_id: auth_ticket.auth_ticket_id,
            user_id: auth_ticket.user_id,
            granted_roles: auth_ticket.granted_roles,
        }
    }
}

impl AuthJwtClaims {
    pub fn into_auth_ticket(self) -> AuthTicket {
        AuthTicket::from_extract(AuthTicketExtract {
            auth_ticket_id: self.auth_ticket_id,
            user_id: self.user_id,
            granted_roles: self.granted_roles,
        })
    }
}
