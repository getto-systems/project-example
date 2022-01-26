use chrono::{DateTime, Utc};

use crate::{
    auth::ticket::remote::kernel::data::{AuthDateTime, AuthNonce, ExpireDateTime},
    z_lib::remote::repository::data::{RegisterResult, RepositoryError},
};

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
