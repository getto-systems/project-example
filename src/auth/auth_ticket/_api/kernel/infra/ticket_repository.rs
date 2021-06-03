use std::{collections::HashMap, sync::Mutex};

use super::{AuthTicketRepository, AuthTicketTokens};

use super::super::data::{
    AuthDateTime, AuthTicket, AuthTicketId, AuthToken, AuthTokenValue, ExpansionLimitDateTime,
};
use crate::z_details::_api::repository::data::RepositoryError;

pub type MemoryAuthTicketStore = Mutex<Store>;

pub struct Store {
    ticket: HashMap<String, Entry>,
    tokens: HashMap<String, Vec<AuthToken>>,
    token_map: HashMap<String, AuthTicketId>,
}

struct Entry {
    limit: ExpansionLimitDateTime,
}

pub struct MemoryAuthTicketRepository<'a> {
    store: &'a MemoryAuthTicketStore,
}

impl<'a> MemoryAuthTicketRepository<'a> {
    pub fn new_store() -> MemoryAuthTicketStore {
        Mutex::new(Store {
            ticket: HashMap::new(),
            tokens: HashMap::new(),
            token_map: HashMap::new(),
        })
    }

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

        let id = ticket.into_id();
        let tokens = tokens.extract();

        store.tokens.insert(id.as_str().into(), tokens.clone());

        tokens.into_iter().for_each(|token| {
            store.token_map.insert(token.as_str().into(), id.clone());
        });

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
    fn disable(&self, token: &AuthTokenValue) -> Result<(), RepositoryError> {
        let mut store = self.store.lock().unwrap();

        let id = store
            .token_map
            .get(token.as_str())
            .map(|id| id.clone())
            .ok_or_else(|| RepositoryError::InfraError(format!("{}", "token not registered")))?;

        store.ticket.remove(id.as_str());
        if let Some(tokens) = store.tokens.remove(id.as_str()) {
            tokens.iter().for_each(|token| {
                store.token_map.remove(token.as_str());
            })
        }
        Ok(())
    }
}
