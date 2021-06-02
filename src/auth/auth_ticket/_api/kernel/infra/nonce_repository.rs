use std::{collections::HashMap, sync::Mutex};

use super::{AuthNonceEntry, AuthNonceRepository};

use super::super::super::kernel::data::ExpireDateTime;
use super::super::data::AuthNonceValue;
use crate::z_details::_api::repository::data::RepositoryError;

pub type MemoryAuthNonceStore = Mutex<HashMap<String, ExpireDateTime>>;

pub struct MemoryAuthNonceRepository<'a> {
    store: &'a MemoryAuthNonceStore,
}

impl<'a> MemoryAuthNonceRepository<'a> {
    pub fn new_store() -> MemoryAuthNonceStore {
        Mutex::new(HashMap::new())
    }

    pub const fn new(store: &'a MemoryAuthNonceStore) -> Self {
        Self { store }
    }
}

impl<'a> AuthNonceRepository for MemoryAuthNonceRepository<'a> {
    fn get(&self, nonce: &AuthNonceValue) -> Result<Option<AuthNonceEntry>, RepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(nonce.as_str()).map(|expires| {
            AuthNonceEntry::new(nonce.clone(), expires.clone())
        }))
    }
    fn put(&self, entry: AuthNonceEntry) -> Result<(), RepositoryError> {
        let mut store = self.store.lock().unwrap();
        store.insert(entry.nonce.as_str().into(), entry.expires);
        Ok(())
    }
}
