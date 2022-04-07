mod ticket;

use crate::auth::x_outside_feature::feature::AuthOutsideStore;

use crate::auth::ticket::kernel::init::ticket_repository::dynamodb::ticket::TableTicket;

use crate::auth::ticket::{
    encode::infra::EncodeAuthTicketRepository, issue::infra::IssueAuthTicketRepository,
    logout::infra::LogoutAuthTicketRepository,
};

use crate::{
    auth::ticket::kernel::data::{AuthDateTime, AuthTicket, ExpansionLimitDateTime},
    z_lib::repository::data::RepositoryError,
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
impl<'a> IssueAuthTicketRepository for DynamoDbAuthTicketRepository<'a> {
    async fn issue(
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
        self.ticket.delete_ticket(ticket.clone()).await
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
