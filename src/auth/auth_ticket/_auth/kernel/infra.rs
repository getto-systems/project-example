use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    auth::auth_ticket::{
        _auth::kernel::data::{AuthDateTime, AuthTicket, ExpansionLimitDateTime, ExpireDateTime},
        _common::kernel::data::AuthTicketExtract,
    },
    z_details::_common::repository::data::RepositoryError,
};

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
