use chrono::{DateTime, Utc};

use crate::{
    auth::{
        proxy::data::AuthProxyError,
        ticket::kernel::data::{
            AuthDateTime, AuthNonce, AuthTicketExtract, AuthToken, DecodeAuthTokenError,
            ExpireDateTime,
        },
        user::kernel::data::RequireAuthRoles,
    },
    z_lib::{
        repository::data::{RegisterResult, RepositoryError},
        request::data::MetadataError,
    },
};

pub struct AuthMetadataContent {
    pub nonce: Option<AuthNonce>,
    pub token: Option<AuthToken>,
}

pub trait AuthMetadata {
    fn metadata(&self) -> Result<AuthMetadataContent, MetadataError>;
}

pub trait AuthNonceMetadata {
    fn nonce(&self) -> Result<Option<AuthNonce>, MetadataError>;
}
pub trait AuthTokenMetadata {
    fn token(&self) -> Result<Option<AuthToken>, MetadataError>;
}

pub trait AuthTokenDecoder {
    fn decode(&self, token: &AuthToken) -> Result<AuthTicketExtract, DecodeAuthTokenError>;
}

pub trait ValidateApiTokenRequestDecoder {
    fn decode(self) -> RequireAuthRoles;
}

#[async_trait::async_trait]
pub trait ValidateService {
    async fn validate(
        &self,
        metadata: AuthMetadataContent,
        require_roles: RequireAuthRoles,
    ) -> Result<(), AuthProxyError>;
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
    expires: ExpireDateTime,
}

impl AuthNonceEntry {
    pub const fn new(nonce: AuthNonce, expires: ExpireDateTime) -> Self {
        Self { nonce, expires }
    }

    #[cfg(test)]
    pub fn nonce(&self) -> &AuthNonce {
        &self.nonce
    }

    pub fn extract(self) -> AuthNonceEntryExtract {
        AuthNonceEntryExtract {
            nonce: self.nonce.extract(),
            expires: self.expires.extract(),
        }
    }
}

#[derive(Clone)]
pub struct AuthNonceEntryExtract {
    pub nonce: String,
    pub expires: DateTime<Utc>,
}

impl From<AuthNonceEntryExtract> for AuthNonceEntry {
    fn from(src: AuthNonceEntryExtract) -> Self {
        Self {
            nonce: AuthNonce::restore(src.nonce),
            expires: ExpireDateTime::restore(src.expires),
        }
    }
}
