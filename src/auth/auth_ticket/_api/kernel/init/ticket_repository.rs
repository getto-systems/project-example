use std::{collections::HashMap, sync::Mutex};

use crate::z_details::_api::repository::helper::register_conflict_error;

use crate::auth::auth_ticket::_api::kernel::infra::AuthTicketRepository;

use crate::auth::auth_ticket::_api::kernel::data::{
    AuthDateTime, AuthTicket, AuthTicketId, ExpansionLimitDateTime,
};
use crate::z_details::_api::repository::data::RepositoryError;

pub struct MemoryAuthTicketRepository<'a> {
    store: &'a MemoryAuthTicketStore,
}

impl<'a> MemoryAuthTicketRepository<'a> {
    pub const fn new(store: &'a MemoryAuthTicketStore) -> Self {
        Self { store }
    }
}

pub type MemoryAuthTicketStore = Mutex<MemoryAuthTicketMap>;
pub struct MemoryAuthTicketMap {
    ticket: HashMap<String, Entry>,
}

struct Entry {
    limit: ExpansionLimitDateTime,
}

impl MemoryAuthTicketMap {
    pub fn new() -> Self {
        Self {
            ticket: HashMap::new(),
        }
    }

    pub fn with_ticket(ticket_id: AuthTicketId, limit: ExpansionLimitDateTime) -> Self {
        let mut store = Self::new();
        store.ticket.insert(ticket_id.extract(), Entry { limit });
        store
    }

    pub fn to_store(self) -> MemoryAuthTicketStore {
        Mutex::new(self)
    }

    fn get(&self, ticket: &AuthTicket) -> Option<&Entry> {
        self.ticket.get(ticket.id_as_str())
    }
    fn insert(&mut self, ticket: AuthTicket, entry: Entry) {
        let ticket = ticket.extract();
        self.ticket.insert(ticket.ticket_id, entry);
    }
}

impl<'a> AuthTicketRepository for MemoryAuthTicketRepository<'a> {
    fn issue(
        &self,
        ticket: AuthTicket,
        limit: ExpansionLimitDateTime,
        _issued_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        let mut store = self.store.lock().unwrap();

        if store.get(&ticket).is_some() {
            return Err(register_conflict_error(ticket.id_as_str()));
        }

        // 実際のデータベースには user_id と registered_at も保存する必要がある
        store.insert(ticket, Entry { limit });

        return Ok(());
    }

    fn discard(
        &self,
        auth_ticket: AuthTicket,
        _discard_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        let mut store = self.store.lock().unwrap();

        store.ticket.remove(auth_ticket.id_as_str());

        // 実際のデータベースには discard_at も保存する必要がある

        return Ok(());
    }

    fn expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
        let store = self.store.lock().unwrap();

        Ok(store
            .ticket
            .get(ticket.id_as_str())
            .map(|entry| entry.limit.clone()))
    }
}
