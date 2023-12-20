use crate::auth::kernel::infra::AuthClock;

use crate::{
    auth::{
        kernel::data::{ExpansionLimitDateTime, ExpireDateTime, ExpireDuration},
        ticket::{
            encode::data::{
                AuthTokenExpires, EncodeAuthTokenError, EncodeAuthTokenSuccess, EncodeTokenError,
            },
            kernel::data::{AuthTicket, AuthenticateToken, AuthorizeToken, CdnToken},
        },
    },
    common::api::repository::data::RepositoryError,
};

pub trait EncodeAuthTokenInfra {
    type Clock: AuthClock;
    type Repository: EncodeAuthTokenRepository;
    type AuthenticateEncoder: AuthenticateTokenEncoder;
    type AuthorizeEncoder: AuthorizeTokenEncoder;
    type CdnEncoder: CdnTokenEncoder;

    fn clock(&self) -> &Self::Clock;
    fn repository(&self) -> &Self::Repository;
    fn authenticate_encoder(&self) -> &Self::AuthenticateEncoder;
    fn authorize_encoder(&self) -> &Self::AuthorizeEncoder;
    fn cdn_encoder(&self) -> &Self::CdnEncoder;
    fn config(&self) -> &EncodeAuthTokenConfig;
}

#[async_trait::async_trait]
pub trait EncodeAuthTokenRepository {
    async fn lookup_expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError>;
}

pub trait AuthenticateTokenEncoder {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<(AuthenticateToken, ExpireDateTime), EncodeTokenError>;
}

pub trait AuthorizeTokenEncoder {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<(AuthorizeToken, ExpireDateTime), EncodeTokenError>;
}

pub trait CdnTokenEncoder {
    fn encode(
        &self,
        expires: ExpireDateTime,
    ) -> Result<(CdnToken, ExpireDateTime), EncodeTokenError>;
}

#[derive(Clone)]
pub struct EncodeAuthTokenConfig {
    pub authenticate_expires: ExpireDuration,
    pub authorize_expires: ExpireDuration,
    pub cdn_expires: ExpireDuration,
}

pub trait EncodeAuthTokenLogger: Send + Sync {
    fn try_to_encode_auth_token(&self);
    fn calculate_token_expires(&self, expires: AuthTokenExpires) -> AuthTokenExpires;
    fn failed_to_lookup_expansion_limit(&self, err: RepositoryError) -> RepositoryError;
    fn expansion_limit_not_found(&self, err: EncodeAuthTokenError) -> EncodeAuthTokenError;
    fn failed_to_encode_token(&self, err: EncodeTokenError) -> EncodeTokenError;
    fn succeed_to_encode_auth_token(&self, auth: EncodeAuthTokenSuccess) -> EncodeAuthTokenSuccess;
}
