use std::{collections::HashMap, sync::Mutex};

use super::AuthTicketRepository;

use super::super::data::{AuthDateTime, AuthTicket, AuthTicketId, ExpansionLimitDateTime};
use crate::z_details::_api::repository::data::{RegisterAttemptResult, RepositoryError};

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
        store.insert(ticket_id, Entry { limit });
        store
    }

    pub fn to_store(self) -> MemoryAuthTicketStore {
        Mutex::new(self)
    }

    fn get(&self, ticket_id: &AuthTicketId) -> Option<&Entry> {
        self.ticket.get(ticket_id.as_str())
    }
    fn insert(&mut self, ticket_id: AuthTicketId, entry: Entry) {
        self.ticket.insert(ticket_id.extract(), entry);
    }
}

impl<'a> AuthTicketRepository for MemoryAuthTicketRepository<'a> {
    fn register(
        &self,
        ticket_id: AuthTicketId,
        limit: ExpansionLimitDateTime,
        _registered_at: AuthDateTime,
    ) -> Result<RegisterAttemptResult<AuthTicketId>, RepositoryError> {
        let mut store = self.store.lock().unwrap();

        if store.get(&ticket_id).is_some() {
            return Ok(RegisterAttemptResult::Conflict);
        }

        // 実際のデータベースには registered_at も保存する必要がある
        store.insert(ticket_id.clone(), Entry { limit });

        return Ok(RegisterAttemptResult::Success(ticket_id));
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
