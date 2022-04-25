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

pub trait AuthorizeRequestDecoder {
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
    async fn register(
        &self,
        nonce: AuthNonce,
        expires: ExpireDateTime,
        registered_at: AuthDateTime,
    ) -> Result<RegisterResult<()>, RepositoryError>;
}
