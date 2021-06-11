use std::{collections::HashMap, sync::Mutex};

use super::{AuthNonceEntry, AuthNonceRepository};

use super::super::super::kernel::data::ExpireDateTime;
use super::super::data::AuthNonceValue;
use crate::z_details::_api::repository::data::RepositoryError;

pub type MemoryAuthNonceStore = Mutex<MemoryAuthNonceMap>;
pub struct MemoryAuthNonceMap(HashMap<String, ExpireDateTime>);

impl MemoryAuthNonceMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_nonce(nonce: String, expires: ExpireDateTime) -> Self {
        let mut hash_map = HashMap::new();
        hash_map.insert(nonce, expires);
        Self(hash_map)
    }

    pub fn to_store(self) -> MemoryAuthNonceStore {
        Mutex::new(self)
    }

    fn get(&self, nonce: &str) -> Option<&ExpireDateTime> {
        self.0.get(nonce)
    }
    fn insert(&mut self, nonce: String, expires: ExpireDateTime) {
        self.0.insert(nonce, expires);
    }
}

pub struct MemoryAuthNonceRepository<'a> {
    store: &'a MemoryAuthNonceStore,
}

impl<'a> MemoryAuthNonceRepository<'a> {
    pub const fn new(store: &'a MemoryAuthNonceStore) -> Self {
        Self { store }
    }
}

impl<'a> AuthNonceRepository for MemoryAuthNonceRepository<'a> {
    fn get(&self, nonce: &AuthNonceValue) -> Result<Option<AuthNonceEntry>, RepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store
            .get(nonce.as_str())
            .map(|expires| AuthNonceEntry::new(nonce.clone(), expires.clone())))
    }
    fn put(&self, entry: AuthNonceEntry) -> Result<(), RepositoryError> {
        let mut store = self.store.lock().unwrap();
        store.insert(entry.nonce.as_str().into(), entry.expires);
        Ok(())
    }
}
