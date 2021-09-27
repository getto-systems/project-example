use chrono::{DateTime, Utc};

use crate::auth::auth_ticket::remote::kernel::infra::{AuthClock, AuthNonceMetadata};

use crate::{
    auth::auth_ticket::remote::kernel::data::{
        AuthDateTime, AuthNonce, ExpireDateTime, ExpireDuration,
    },
    z_details::_common::repository::data::{RegisterResult, RepositoryError},
};

pub trait CheckAuthNonceInfra {
    type Clock: AuthClock;
    type NonceMetadata: AuthNonceMetadata;
    type NonceRepository: AuthNonceRepository;

    fn clock(&self) -> &Self::Clock;
    fn nonce_metadata(&self) -> &Self::NonceMetadata;
    fn nonce_repository(&self) -> &Self::NonceRepository;
    fn config(&self) -> &AuthNonceConfig;
}

#[async_trait::async_trait]
pub trait AuthNonceRepository {
    async fn put(
        &self,
        nonce: AuthNonceEntry,
        registered_at: AuthDateTime,
    ) -> Result<RegisterResult<()>, RepositoryError>;
}

pub struct AuthNonceEntry {
    nonce: AuthNonce,
    expires: Option<ExpireDateTime>,
}

impl AuthNonceEntry {
    pub const fn new(nonce: AuthNonce, expires: ExpireDateTime) -> Self {
        Self {
            nonce,
            expires: Some(expires),
        }
    }

    #[cfg(test)]
    pub fn nonce(&self) -> &AuthNonce {
        &self.nonce
    }

    pub fn extract(self) -> AuthNonceEntryExtract {
        AuthNonceEntryExtract {
            nonce: self.nonce.extract(),
            expires: self.expires.map(|expires| expires.extract()),
        }
    }
}

#[derive(Clone)]
pub struct AuthNonceEntryExtract {
    pub nonce: String,
    pub expires: Option<DateTime<Utc>>,
}

impl From<AuthNonceEntryExtract> for AuthNonceEntry {
    fn from(src: AuthNonceEntryExtract) -> Self {
        Self {
            nonce: AuthNonce::restore(src.nonce),
            expires: src.expires.map(|expires| ExpireDateTime::restore(expires)),
        }
    }
}

pub struct AuthNonceConfig {
    pub nonce_expires: ExpireDuration,
}
