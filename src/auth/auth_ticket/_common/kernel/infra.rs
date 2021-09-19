use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    auth::auth_ticket::_common::kernel::data::{
        AuthNonce, AuthServiceMetadataError, AuthTicketExtract, AuthToken, DecodeAuthTokenError,
    },
    z_details::_common::request::data::MetadataError,
};

pub struct AuthServiceMetadataContent {
    // TODO Option じゃなくできるはず
    pub nonce: Option<AuthNonce>,
    // TODO こっちは unauthorized なリクエストがあるんだけどなんとかならんかね？
    pub token: Option<AuthToken>,
}

pub trait AuthServiceMetadata {
    fn metadata(&self) -> Result<AuthServiceMetadataContent, AuthServiceMetadataError>;
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

pub struct TicketJwtClaims(AuthTicketExtract, DateTime<Utc>);

impl Into<(AuthJwtClaims, i64)> for TicketJwtClaims {
    fn into(self) -> (AuthJwtClaims, i64) {
        to_claims(AUTH_JWT_AUDIENCE_TICKET.into(), self.0, self.1)
    }
}

pub struct ApiJwtClaims(AuthTicketExtract, DateTime<Utc>);

impl Into<(AuthJwtClaims, i64)> for ApiJwtClaims {
    fn into(self) -> (AuthJwtClaims, i64) {
        to_claims(AUTH_JWT_AUDIENCE_API.into(), self.0, self.1)
    }
}

fn to_claims(
    aud: String,
    ticket: AuthTicketExtract,
    expires: DateTime<Utc>,
) -> (AuthJwtClaims, i64) {
    let exp = expires.timestamp();
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
