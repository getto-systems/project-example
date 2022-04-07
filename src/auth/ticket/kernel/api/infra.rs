use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::auth::ticket::kernel::data::{
    AuthDateTime, AuthTicket, AuthTicketExtract, AuthTokenMessage, AuthTokenResponse,
    ExpireDateTime,
};

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
fn to_claims(aud: String, ticket: AuthTicket, expires: ExpireDateTime) -> (AuthJwtClaims, i64) {
    let (ticket_id, user) = ticket.extract();
    let user = user.extract();
    let exp = expires.extract().timestamp();
    (
        AuthJwtClaims {
            aud,
            exp,
            ticket_id: ticket_id.extract(),
            user_id: user.user_id,
            granted_roles: user.granted_roles,
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
