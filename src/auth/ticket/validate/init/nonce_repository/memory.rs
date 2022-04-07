mod nonce;

use crate::auth::ticket::validate::init::nonce_repository::memory::nonce::{
    EntryNonce, MapNonce, StoreNonce,
};

use crate::auth::ticket::validate::infra::AuthNonceRepository;

use crate::{
    auth::ticket::kernel::data::{AuthDateTime, AuthNonce, ExpireDateTime},
    z_lib::repository::data::{RegisterResult, RepositoryError},
};

pub struct MemoryAuthNonceRepository<'a> {
    nonce: MapNonce<'a>,
}

pub struct MemoryAuthNonceStore {
    nonce: StoreNonce,
}

impl MemoryAuthNonceStore {
    pub fn new() -> Self {
        Self {
            nonce: MapNonce::new_store(),
        }
    }
}

impl<'a> MemoryAuthNonceRepository<'a> {
    pub fn new(store: &'a MemoryAuthNonceStore) -> Self {
        Self {
            nonce: MapNonce::new(&store.nonce),
        }
    }

    pub fn with_nonce(
        store: &'a MemoryAuthNonceStore,
        nonce: AuthNonce,
        expires: ExpireDateTime,
        registered_at: AuthDateTime,
    ) -> Self {
        let repository = Self::new(store);
        repository.nonce.insert_entry(
            nonce,
            EntryNonce {
                expires,
                registered_at,
            },
        );
        repository
    }
}

#[async_trait::async_trait]
impl<'a> AuthNonceRepository for MemoryAuthNonceRepository<'a> {
    async fn register(
        &self,
        nonce: AuthNonce,
        expires: ExpireDateTime,
        registered_at: AuthDateTime,
    ) -> Result<RegisterResult<()>, RepositoryError> {
        Ok(self.nonce.insert_nonce(nonce, expires, registered_at))
    }
}
