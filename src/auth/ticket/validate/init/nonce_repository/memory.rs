mod nonce;

use crate::auth::ticket::validate::init::nonce_repository::memory::nonce::{EntryNonce, MapNonce};

use crate::auth::ticket::validate::infra::AuthNonceRepository;

use crate::{
    auth::ticket::kernel::data::{AuthDateTime, AuthNonce, ExpireDateTime},
    z_lib::repository::data::{RegisterResult, RepositoryError},
};

pub struct MemoryAuthNonceRepository {
    nonce: MapNonce,
}

impl MemoryAuthNonceRepository {
    pub fn new() -> Self {
        Self {
            nonce: MapNonce::new(),
        }
    }

    pub fn with_nonce(nonce: AuthNonce, expires: ExpireDateTime, registered_at: AuthDateTime) -> Self {
        let repository = Self::new();
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
impl AuthNonceRepository for MemoryAuthNonceRepository {
    async fn register(
        &self,
        nonce: AuthNonce,
        expires: ExpireDateTime,
        registered_at: AuthDateTime,
    ) -> Result<RegisterResult<()>, RepositoryError> {
        Ok(self.nonce.insert_nonce(nonce, expires, registered_at))
    }
}
