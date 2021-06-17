use std::{collections::HashMap, sync::Mutex};

use super::{AuthTicketIdGenerator, AuthTicketRepository};

use super::super::data::{AuthDateTime, AuthTicket, AuthTicketId, ExpansionLimitDateTime};
use crate::z_details::_api::repository::data::RepositoryError;

pub type MemoryAuthTicketStore = Mutex<MemoryAuthTicketMap>;
pub struct MemoryAuthTicketMap {
    ticket: HashMap<String, Entry>,
}

impl MemoryAuthTicketMap {
    pub fn new() -> Self {
        Self {
            ticket: HashMap::new(),
        }
    }

    pub fn with_ticket(ticket_id: String, limit: ExpansionLimitDateTime) -> Self {
        let mut ticket = HashMap::new();
        ticket.insert(ticket_id, Entry { limit });

        Self { ticket }
    }

    pub fn to_store(self) -> MemoryAuthTicketStore {
        Mutex::new(self)
    }
}

struct Entry {
    limit: ExpansionLimitDateTime,
}

pub struct MemoryAuthTicketRepository<'a> {
    store: &'a MemoryAuthTicketStore,
}

impl<'a> MemoryAuthTicketRepository<'a> {
    pub const fn new(store: &'a MemoryAuthTicketStore) -> Self {
        Self { store }
    }
}

const REGISTER_TRY_LIMIT: u8 = 10;

impl<'a> AuthTicketRepository for MemoryAuthTicketRepository<'a> {
    fn register(
        &self,
        id_generator: &impl AuthTicketIdGenerator,
        _registered_at: AuthDateTime,
        limit: ExpansionLimitDateTime,
    ) -> Result<AuthTicketId, RepositoryError> {
        let mut store = self.store.lock().unwrap();
        let mut count = 0;

        loop {
            let id = id_generator.generate();

            if store.ticket.get(id.as_str()).is_some() {
                count += 1;
                if count > REGISTER_TRY_LIMIT {
                    return Err(RepositoryError::InfraError(format!(
                        "the maximum number of registration attempts has been reached; limit: {}",
                        REGISTER_TRY_LIMIT
                    )));
                }
                continue;
            }

            // 実際のデータベースには registered_at も保存する必要がある
            store.ticket.insert(id.as_str().into(), Entry { limit });

            return Ok(id);
        }
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
