use std::{collections::HashMap, sync::Mutex};

use crate::auth::ticket::validate::infra::{
    AuthNonceEntry, AuthNonceEntryExtract, AuthNonceRepository,
};

use crate::{
    auth::ticket::kernel::data::{AuthDateTime, AuthNonce, ExpireDateTime},
    z_lib::repository::data::{RegisterResult, RepositoryError},
};

pub type MemoryAuthNonceStore = Mutex<MemoryAuthNonceMap>;
pub struct MemoryAuthNonceMap(HashMap<String, AuthNonceEntryExtract>);

impl MemoryAuthNonceMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_nonce(nonce: String, expires: ExpireDateTime) -> Self {
        let mut hash_map = HashMap::new();
        hash_map.insert(
            nonce.clone(),
            AuthNonceEntryExtract {
                nonce,
                expires: expires.extract(),
            },
        );
        Self(hash_map)
    }

    pub fn to_store(self) -> MemoryAuthNonceStore {
        Mutex::new(self)
    }

    fn get(&self, nonce: &AuthNonce) -> Option<&AuthNonceEntryExtract> {
        self.0.get(nonce.as_str())
    }
    fn insert(&mut self, entry: AuthNonceEntry) {
        let extract = entry.extract();
        self.0.insert(extract.nonce.clone(), extract);
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

#[async_trait::async_trait]
impl<'a> AuthNonceRepository for MemoryAuthNonceRepository<'a> {
    async fn put(
        &self,
        entry: AuthNonceEntry,
        registered_at: AuthDateTime,
    ) -> Result<RegisterResult<()>, RepositoryError> {
        put_nonce(self, entry, registered_at)
    }
}
fn put_nonce<'a>(
    repository: &MemoryAuthNonceRepository<'a>,
    entry: AuthNonceEntry,
    registered_at: AuthDateTime,
) -> Result<RegisterResult<()>, RepositoryError> {
    let mut store = repository.store.lock().unwrap();

    if let Some(found) = store.get(entry.nonce()) {
        if !ExpireDateTime::restore(found.expires).has_elapsed(&registered_at) {
            return Ok(RegisterResult::Conflict);
        }
    }

    store.insert(entry);
    Ok(RegisterResult::Success(()))
}
