mod ticket;

use crate::auth::ticket::authorize::infra::ClarifyAuthorizeTokenAuthTicketRepository;
use crate::auth::ticket::kernel::init::ticket_repository::memory::ticket::{
    EntryTicket, MapTicket, StoreTicket,
};

use crate::auth::{
    ticket::{
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
        let repository = Self::new(store);
        repository.ticket.insert_entry(
            ticket.ticket_id,
            EntryTicket {
                user_id: ticket.attrs.user_id,
                limit,
                issued_at,
            },
        );
        repository
    }
}

#[async_trait::async_trait]
impl<'a> ClarifyAuthorizeTokenAuthTicketRepository for MemoryAuthTicketRepository<'a> {
    async fn lookup_expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
        Ok(self.ticket.get_expansion_limit(&ticket.ticket_id))
    }
}

#[async_trait::async_trait]
impl<'a> IssueAuthTicketRepository for MemoryAuthTicketRepository<'a> {
    async fn register(
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
        Ok(self.ticket.remove_ticket(&ticket.ticket_id))
    }
}

#[async_trait::async_trait]
impl<'a> EncodeAuthTicketRepository for MemoryAuthTicketRepository<'a> {
    async fn lookup_expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
        Ok(self.ticket.get_expansion_limit(&ticket.ticket_id))
    }
}

#[async_trait::async_trait]
impl<'a> DiscardAuthTicketRepository for MemoryAuthTicketRepository<'a> {
    async fn discard_all(&self, user_id: &AuthUserId) -> Result<(), RepositoryError> {
        for ticket_id in self.ticket.get_all_ticket_id(user_id) {
            self.ticket.remove_ticket(&ticket_id)
        }
        Ok(())
    }
}
