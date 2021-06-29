use std::{collections::HashMap, sync::Mutex};

use crate::auth::auth_ticket::_api::kernel::infra::{AuthNonceEntry, AuthNonceRepository};

use crate::auth::auth_ticket::_api::kernel::data::{AuthNonceValue, ExpireDateTime};
use crate::z_details::_api::repository::data::RepositoryError;

pub type MemoryAuthNonceStore = Mutex<MemoryAuthNonceMap>;
pub struct MemoryAuthNonceMap(HashMap<String, AuthNonceEntry>);

impl MemoryAuthNonceMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_nonce(nonce: String, expires: ExpireDateTime) -> Self {
        let mut hash_map = HashMap::new();
        hash_map.insert(
            nonce.clone(),
            AuthNonceEntry::new(AuthNonceValue::new(nonce), expires),
        );
        Self(hash_map)
    }

    pub fn to_store(self) -> MemoryAuthNonceStore {
        Mutex::new(self)
    }

    fn get(&self, nonce: &AuthNonceValue) -> Option<&AuthNonceEntry> {
        self.0.get(nonce.as_str())
    }
    fn insert(&mut self, entry: AuthNonceEntry) {
        self.0.insert(entry.clone().into_nonce().extract(), entry);
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
        Ok(store.get(nonce).map(|entry| entry.clone()))
    }
    fn put(&self, entry: AuthNonceEntry) -> Result<(), RepositoryError> {
        let mut store = self.store.lock().unwrap();
        store.insert(entry);
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use std::{collections::HashMap, sync::Mutex};

    use crate::auth::auth_ticket::_api::kernel::infra::{AuthNonceEntry, AuthNonceRepository};

    use crate::auth::auth_ticket::_api::kernel::data::{AuthNonceValue, ExpireDateTime};
    use crate::z_details::_api::repository::data::RepositoryError;

    pub type MemoryAuthNonceStore = Mutex<MemoryAuthNonceMap>;
    pub struct MemoryAuthNonceMap(HashMap<String, AuthNonceEntry>);

    impl MemoryAuthNonceMap {
        pub fn new() -> Self {
            Self(HashMap::new())
        }

        pub fn with_nonce(nonce: String, expires: ExpireDateTime) -> Self {
            let mut hash_map = HashMap::new();
            hash_map.insert(
                nonce.clone(),
                AuthNonceEntry::new(AuthNonceValue::new(nonce), expires),
            );
            Self(hash_map)
        }

        pub fn to_store(self) -> MemoryAuthNonceStore {
            Mutex::new(self)
        }

        fn get(&self, nonce: &AuthNonceValue) -> Option<&AuthNonceEntry> {
            self.0.get(nonce.as_str())
        }
        fn insert(&mut self, entry: AuthNonceEntry) {
            self.0.insert(entry.clone().into_nonce().extract(), entry);
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
            Ok(store.get(nonce).map(|entry| entry.clone()))
        }
        fn put(&self, entry: AuthNonceEntry) -> Result<(), RepositoryError> {
            let mut store = self.store.lock().unwrap();
            store.insert(entry);
            Ok(())
        }
    }
}
