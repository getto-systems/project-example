mod ticket;

use crate::auth::x_outside_feature::feature::AuthOutsideStore;

use crate::auth::ticket::kernel::init::ticket_repository::dynamodb::ticket::TableTicket;

use crate::auth::{
    ticket::{
        authorize::infra::ClarifyAuthorizeTokenAuthTicketRepository,
        encode::infra::EncodeAuthTicketRepository, issue::infra::IssueAuthTicketRepository,
        logout::infra::LogoutAuthTicketRepository,
    },
    user::account::unregister::infra::DiscardAuthTicketRepository,
};

use crate::{
    auth::{
        kernel::data::{AuthDateTime, ExpansionLimitDateTime},
        ticket::kernel::data::AuthTicket,
        user::kernel::data::AuthUserId,
    },
    common::api::repository::data::RepositoryError,
};

pub struct DynamoDbAuthTicketRepository<'a> {
    ticket: TableTicket<'a>,
}

impl<'a> DynamoDbAuthTicketRepository<'a> {
    pub const fn new(feature: &'a AuthOutsideStore) -> Self {
        Self {
            ticket: TableTicket::new(feature),
        }
    }
}

#[async_trait::async_trait]
impl<'a> ClarifyAuthorizeTokenAuthTicketRepository for DynamoDbAuthTicketRepository<'a> {
    async fn lookup_expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
        self.ticket.get_expansion_limit(ticket.clone()).await
    }
}

#[async_trait::async_trait]
impl<'a> IssueAuthTicketRepository for DynamoDbAuthTicketRepository<'a> {
    async fn register(
        &self,
        ticket: AuthTicket,
        expansion_limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        self.ticket
            .put_ticket(ticket, expansion_limit, issued_at)
            .await
    }
}

#[async_trait::async_trait]
impl<'a> LogoutAuthTicketRepository for DynamoDbAuthTicketRepository<'a> {
    async fn discard(&self, ticket: &AuthTicket) -> Result<(), RepositoryError> {
        let ticket = ticket.clone();
        self.ticket
            .delete_ticket(ticket.ticket_id, ticket.attrs.user_id)
            .await
    }
}

#[async_trait::async_trait]
impl<'a> EncodeAuthTicketRepository for DynamoDbAuthTicketRepository<'a> {
    async fn lookup_expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
        self.ticket.get_expansion_limit(ticket.clone()).await
    }
}

#[async_trait::async_trait]
impl<'a> DiscardAuthTicketRepository for DynamoDbAuthTicketRepository<'a> {
    async fn discard_all(&self, user_id: &AuthUserId) -> Result<(), RepositoryError> {
        for ticket_id in self.ticket.query_ticket_id(user_id.clone()).await? {
            self.ticket
                .delete_ticket(ticket_id.clone(), user_id.clone())
                .await?
        }
        Ok(())
    }
}
