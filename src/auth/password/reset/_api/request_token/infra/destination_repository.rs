use std::{collections::HashMap, sync::Mutex};

use super::ResetTokenDestinationRepository;

use crate::{
    auth::{
        login_id::_api::data::LoginId,
        password::reset::_api::request_token::data::ResetTokenDestination,
    },
    z_details::_api::repository::data::RepositoryError,
};

pub type MemoryResetTokenDestinationStore = Mutex<MemoryResetTokenDestinationMap>;
pub struct MemoryResetTokenDestinationMap(HashMap<String, ResetTokenDestination>);

impl MemoryResetTokenDestinationMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_destination(login_id: LoginId, destination: ResetTokenDestination) -> Self {
        let mut store = Self::new();
        store.insert(login_id, destination);
        store
    }

    pub fn to_store(self) -> MemoryResetTokenDestinationStore {
        Mutex::new(self)
    }

    fn insert(&mut self, login_id: LoginId, destination: ResetTokenDestination) {
        self.0.insert(login_id.extract(), destination);
    }
    fn get(&self, login_id: &LoginId) -> Option<&ResetTokenDestination> {
        self.0.get(login_id.as_str())
    }
}

pub struct MemoryResetTokenDestinationRepository<'a> {
    store: &'a MemoryResetTokenDestinationStore,
}

impl<'a> MemoryResetTokenDestinationRepository<'a> {
    pub const fn new(store: &'a MemoryResetTokenDestinationStore) -> Self {
        Self { store }
    }
}

impl<'a> ResetTokenDestinationRepository for MemoryResetTokenDestinationRepository<'a> {
    fn get(&self, login_id: &LoginId) -> Result<Option<ResetTokenDestination>, RepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(login_id).map(|destination| destination.clone()))
    }
}
