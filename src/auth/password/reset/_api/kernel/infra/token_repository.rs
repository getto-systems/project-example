use std::{collections::HashMap, sync::Mutex};

use super::{ResetTokenActiveEntry, ResetTokenEntry, ResetTokenGenerator, ResetTokenRepository};

use crate::{
    auth::{
        auth_ticket::_api::kernel::data::{AuthDateTime, ExpireDateTime},
        password::reset::_api::{
            kernel::data::ResetToken, request_token::data::ResetTokenDestination,
        },
    },
    z_details::_api::repository::data::RepositoryError,
};

pub type MemoryResetTokenStore = Mutex<MemoryResetTokenMap>;
pub struct MemoryResetTokenMap {
    active: HashMap<String, ResetTokenActiveEntry>,
    discard: HashMap<String, AuthDateTime>,
}

impl MemoryResetTokenMap {
    pub fn new() -> Self {
        Self {
            active: HashMap::new(),
            discard: HashMap::new(),
        }
    }

    pub fn with_token(
        destination: ResetTokenDestination,
        token: ResetToken,
        expires: ExpireDateTime,
    ) -> Self {
        let mut store = Self::new();
        store.insert(token, ResetTokenActiveEntry::new(destination, expires));
        store
    }

    pub fn to_store(self) -> MemoryResetTokenStore {
        Mutex::new(self)
    }

    fn insert(&mut self, token: ResetToken, entry: ResetTokenActiveEntry) {
        self.active.insert(token.extract(), entry);
    }
    fn get(&self, token: &ResetToken) -> Option<ResetTokenEntry> {
        match self.active.get(token.as_str()) {
            Some(entry) => Some(ResetTokenEntry::Active(entry.clone())),
            None => self
                .discard
                .get(token.as_str())
                .map(|_| ResetTokenEntry::Discarded),
        }
    }
    fn discard(
        &mut self,
        token: ResetToken,
        discard_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        let entry = self.active.remove(token.as_str());
        if entry.is_none() {
            return Err(RepositoryError::InfraError(format!(
                "{}",
                "there is no active token"
            )));
        }
        self.discard.insert(token.extract(), discard_at);
        Ok(())
    }
}

pub struct MemoryResetTokenRepository<'a> {
    store: &'a MemoryResetTokenStore,
}

impl<'a> MemoryResetTokenRepository<'a> {
    pub const fn new(store: &'a MemoryResetTokenStore) -> Self {
        Self { store }
    }
}

const REGISTER_TRY_LIMIT: u8 = 10;

impl<'a> ResetTokenRepository for MemoryResetTokenRepository<'a> {
    fn register(
        &self,
        destination: ResetTokenDestination,
        token_generator: &impl ResetTokenGenerator,
        expires: ExpireDateTime,
        _registered_at: AuthDateTime,
    ) -> Result<ResetToken, RepositoryError> {
        let mut store = self.store.lock().unwrap();
        let mut count = 0;

        loop {
            let token = token_generator.generate();

            if store.get(&token).is_some() {
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
            store.insert(token.clone(), ResetTokenActiveEntry::new(destination, expires));

            return Ok(token);
        }
    }

    fn get(&self, token: &ResetToken) -> Result<Option<ResetTokenEntry>, RepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(token).map(|entry| entry.clone()))
    }

    fn discard(&self, token: ResetToken, discard_at: AuthDateTime) -> Result<(), RepositoryError> {
        let mut store = self.store.lock().unwrap();
        store.discard(token, discard_at)
    }
}
