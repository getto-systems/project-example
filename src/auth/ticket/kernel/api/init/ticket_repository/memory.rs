mod ticket;

use crate::auth::ticket::kernel::init::ticket_repository::memory::ticket::{
    EntryTicket, MapTicket, StoreTicket,
};

use crate::auth::ticket::{
    encode::infra::EncodeAuthTicketRepository, issue::infra::IssueAuthTicketRepository,
    logout::infra::LogoutAuthTicketRepository,
};

use crate::{
    auth::ticket::kernel::data::{AuthDateTime, AuthTicket, ExpansionLimitDateTime},
    z_lib::repository::data::RepositoryError,
};

pub struct MemoryAuthTicketRepository<'a> {
    ticket: MapTicket<'a>,
}

pub struct MemoryAuthTicketStore {
    ticket: StoreTicket,
}

impl MemoryAuthTicketStore {
    pub fn new() -> Self {
        Self {
            ticket: MapTicket::new_store(),
        }
    }
}

impl<'a> MemoryAuthTicketRepository<'a> {
    pub fn new(store: &'a MemoryAuthTicketStore) -> Self {
        Self {
            ticket: MapTicket::new(&store.ticket),
        }
    }

    pub fn with_ticket(
        store: &'a MemoryAuthTicketStore,
        ticket: AuthTicket,
        limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) -> Self {
        let (ticket_id, user) = ticket.extract();

        let repository = Self::new(store);
        repository.ticket.insert_entry(
            ticket_id,
            EntryTicket {
                user,
                limit,
                issued_at,
            },
        );
        repository
    }
}

#[async_trait::async_trait]
impl<'a> IssueAuthTicketRepository for MemoryAuthTicketRepository<'a> {
    async fn issue(
        &self,
        ticket: AuthTicket,
        limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        Ok(self.ticket.insert_ticket(ticket, limit, issued_at))
    }
}

#[async_trait::async_trait]
impl<'a> LogoutAuthTicketRepository for MemoryAuthTicketRepository<'a> {
    async fn discard(&self, ticket: &AuthTicket) -> Result<(), RepositoryError> {
        Ok(self.ticket.remove_ticket(ticket.as_ticket_id()))
    }
}

#[async_trait::async_trait]
impl<'a> EncodeAuthTicketRepository for MemoryAuthTicketRepository<'a> {
    async fn lookup_expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
        Ok(self.ticket.get_expansion_limit(ticket.as_ticket_id()))
    }
}
