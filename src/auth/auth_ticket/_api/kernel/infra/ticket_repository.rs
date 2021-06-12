use std::{collections::HashMap, sync::Mutex};

use super::{AuthTicketRepository, AuthTicketTokens};

use super::super::data::{
    AuthDateTime, AuthTicket, AuthTicketId, AuthToken, ExpansionLimitDateTime,
};
use crate::z_details::_api::repository::data::RepositoryError;

pub type MemoryAuthTicketStore = Mutex<MemoryAuthTicketMap>;
pub struct MemoryAuthTicketMap {
    ticket: HashMap<String, Entry>,
    tokens: HashMap<String, Vec<AuthToken>>,
}

impl MemoryAuthTicketMap {
    pub fn new() -> Self {
        Self {
            ticket: HashMap::new(),
            tokens: HashMap::new(),
        }
    }

    pub fn with_ticket(ticket_id: String, limit: ExpansionLimitDateTime) -> Self {
        let mut ticket = HashMap::new();
        ticket.insert(ticket_id, Entry { limit });

        Self {
            ticket,
            tokens: HashMap::new(),
        }
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
        id_generator: impl Fn() -> AuthTicketId,
        _registered_at: AuthDateTime,
        limit: ExpansionLimitDateTime,
    ) -> Result<AuthTicketId, RepositoryError> {
        let mut store = self.store.lock().unwrap();
        let mut count = 0;

        loop {
            let id = id_generator();

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
    fn register_tokens(
        &self,
        ticket: AuthTicket,
        tokens: AuthTicketTokens,
    ) -> Result<(), RepositoryError> {
        let mut store = self.store.lock().unwrap();

        store
            .tokens
            .insert(ticket.into_id().as_str().into(), tokens.extract());

        Ok(())
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
