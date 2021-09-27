use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    auth::auth_ticket::remote::kernel::data::{
        AuthDateTime, AuthNonce, AuthTicket, AuthTicketExtract, AuthToken, AuthTokenMessage,
        AuthTokenResponse, DecodeAuthTokenError, ExpansionLimitDateTime, ExpireDateTime,
    },
    z_details::_common::{repository::data::RepositoryError, request::data::MetadataError},
};

pub struct AuthMetadataContent {
    pub nonce: Option<AuthNonce>,
    pub token: Option<AuthToken>,
}

pub trait AuthMetadata {
    fn metadata(&self) -> Result<AuthMetadataContent, MetadataError>;
}

pub trait AuthNonceMetadata {
    fn nonce(&self) -> Result<Option<AuthNonce>, MetadataError>;
}
pub trait AuthTokenMetadata {
    fn token(&self) -> Result<Option<AuthToken>, MetadataError>;
}

pub trait AuthTokenDecoder {
    fn decode(&self, token: &AuthToken) -> Result<AuthTicketExtract, DecodeAuthTokenError>;
}

pub trait AuthTokenResponseBuilder {
    fn build(&self, message: AuthTokenMessage) -> AuthTokenResponse;
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
    pub fn new_ticket(ticket: AuthTicket, expires: ExpireDateTime) -> (Self, i64) {
        to_claims(AUTH_JWT_AUDIENCE_TICKET.into(), ticket, expires)
    }
    pub fn new_api(ticket: AuthTicket, expires: ExpireDateTime) -> (Self, i64) {
        to_claims(AUTH_JWT_AUDIENCE_API.into(), ticket, expires)
    }
}
fn to_claims(
    aud: String,
    ticket: AuthTicket,
    expires: ExpireDateTime,
) -> (AuthJwtClaims, i64) {
    let ticket = ticket.extract();
    let exp = expires.extract().timestamp();
    (
        AuthJwtClaims {
            aud,
            exp,
            ticket_id: ticket.ticket_id,
            user_id: ticket.user_id,
            granted_roles: ticket.granted_roles,
        },
        exp,
    )
}

impl Into<AuthTicketExtract> for AuthJwtClaims {
    fn into(self) -> AuthTicketExtract {
        AuthTicketExtract {
            ticket_id: self.ticket_id,
            user_id: self.user_id,
            granted_roles: self.granted_roles,
        }
    }
}

pub trait AuthClock {
    fn now(&self) -> AuthDateTime;
}

#[async_trait::async_trait]
pub trait IssueAuthTicketRepository {
    async fn issue(
        &self,
        ticket: AuthTicket,
        limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) -> Result<(), RepositoryError>;
}

#[async_trait::async_trait]
pub trait DiscardAuthTicketRepository {
    async fn discard(
        &self,
        auth_ticket: AuthTicket,
        discard_at: AuthDateTime,
    ) -> Result<(), RepositoryError>;
}

#[async_trait::async_trait]
pub trait AuthTicketRepository {
    async fn expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError>;
}
