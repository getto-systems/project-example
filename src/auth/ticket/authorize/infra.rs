use crate::{
    auth::{
        kernel::data::ExpansionLimitDateTime,
        ticket::{
            authorize::data::ValidateAuthorizeFieldsError,
            kernel::data::{
                AuthPermissionGranted, AuthPermissionRequired, AuthPermissionRequiredExtract,
                AuthTicket, AuthorizeToken, AuthorizeTokenExtract, DecodeAuthorizeTokenError,
            },
        },
        user::kernel::data::AuthUserId,
    },
    common::api::repository::data::RepositoryError,
};

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
