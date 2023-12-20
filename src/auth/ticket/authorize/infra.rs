use crate::auth::kernel::infra::AuthClock;

use crate::{
    auth::{
        kernel::data::ExpansionLimitDateTime,
        ticket::{
            authorize::data::{AuthorizeError, AuthorizeSuccess, ValidateAuthorizeFieldsError},
            kernel::data::{
                AuthPermissionError, AuthPermissionGranted, AuthPermissionRequired,
                AuthPermissionRequiredExtract, AuthTicket, AuthorizeToken, AuthorizeTokenExtract,
                DecodeAuthorizeTokenError, ValidateAuthorizeTokenError,
            },
        },
        user::kernel::data::AuthUserId,
    },
    common::api::repository::data::RepositoryError,
};

pub trait CheckAuthorizeTokenInfra {
    type TokenDecoder: AuthorizeTokenDecoder;

    fn token_decoder(&self) -> &Self::TokenDecoder;
}

pub trait CheckAuthorizeTokenLogger: Send + Sync {
    fn try_to_check_authorize_token(&self);
    fn invalid_request(&self, err: ValidateAuthorizeTokenError) -> ValidateAuthorizeTokenError;
    fn invalid_token(&self, err: DecodeAuthorizeTokenError) -> DecodeAuthorizeTokenError;
    fn forbidden(&self, err: AuthPermissionError) -> AuthPermissionError;
    fn succeed_to_check_authorize_token(
        &self,
        granted: AuthPermissionGranted,
    ) -> AuthPermissionGranted;
}

pub trait AuthorizeInfra {
    type TokenDecoder: AuthorizeTokenDecoder;
    type Repository: AuthorizeRepository;
    type Clock: AuthClock;

    fn token_decoder(&self) -> &Self::TokenDecoder;
    fn repository(&self) -> &Self::Repository;
    fn clock(&self) -> &Self::Clock;
}

pub trait AuthorizeLogger: Send + Sync {
    fn try_to_authorize(&self);
    fn invalid_request(&self, err: ValidateAuthorizeFieldsError) -> ValidateAuthorizeFieldsError;
    fn invalid_token(&self, err: DecodeAuthorizeTokenError) -> DecodeAuthorizeTokenError;
    fn forbidden(&self, err: AuthPermissionError) -> AuthPermissionError;
    fn failed_to_lookup_expansion_limit(&self, err: RepositoryError) -> RepositoryError;
    fn failed_to_lookup_permission_granted(&self, err: RepositoryError) -> RepositoryError;
    fn expansion_limit_not_found(&self, err: AuthorizeError) -> AuthorizeError;
    fn ticket_has_expired(&self, err: AuthorizeError) -> AuthorizeError;
    fn permission_granted_not_found(&self, err: AuthorizeError) -> AuthorizeError;
    fn authorized(&self, auth: AuthorizeSuccess) -> AuthorizeSuccess;
}

pub trait AuthorizeWithTokenLogger {
    fn invalid_request(&self, err: ValidateAuthorizeFieldsError) -> ValidateAuthorizeFieldsError;
    fn invalid_token(&self, err: DecodeAuthorizeTokenError) -> DecodeAuthorizeTokenError;
    fn forbidden(&self, err: AuthPermissionError) -> AuthPermissionError;
    fn authorized(&self);
}

pub trait AuthorizeTokenDecoder {
    fn decode(&self, token: AuthorizeToken) -> Result<AuthTicket, DecodeAuthorizeTokenError>;
}

pub struct AuthorizeFields {
    pub token: AuthorizeToken,
    pub required: AuthPermissionRequired,
}

pub trait AuthorizeFieldsExtract {
    fn convert(self) -> Result<AuthorizeFields, ValidateAuthorizeFieldsError>;
}

impl AuthorizeFieldsExtract for AuthorizeFields {
    fn convert(self) -> Result<AuthorizeFields, ValidateAuthorizeFieldsError> {
        Ok(self)
    }
}

impl<T: AuthorizeTokenExtract> AuthorizeFieldsExtract for (T, AuthPermissionRequired) {
    fn convert(self) -> Result<AuthorizeFields, ValidateAuthorizeFieldsError> {
        Ok(AuthorizeFields {
            token: self
                .0
                .convert()
                .map_err(ValidateAuthorizeFieldsError::Token)?,
            required: self.1,
        })
    }
}

impl<T: AuthorizeTokenExtract, R: AuthPermissionRequiredExtract> AuthorizeFieldsExtract for (T, R) {
    fn convert(self) -> Result<AuthorizeFields, ValidateAuthorizeFieldsError> {
        Ok(AuthorizeFields {
            token: self
                .0
                .convert()
                .map_err(ValidateAuthorizeFieldsError::Token)?,
            required: self
                .1
                .convert()
                .map_err(ValidateAuthorizeFieldsError::Required)?,
        })
    }
}

#[async_trait::async_trait]
pub trait ClarifyAuthorizeTokenAuthTicketRepository {
    async fn lookup_expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError>;
}

#[async_trait::async_trait]
pub trait ClarifyAuthorizeTokenAuthUserRepository {
    async fn lookup_permission_granted(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<AuthPermissionGranted>, RepositoryError>;
}

#[async_trait::async_trait]
pub trait AuthorizeRepository {
    async fn lookup_expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError>;

    async fn lookup_permission_granted(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<AuthPermissionGranted>, RepositoryError>;
}
