use crate::{
    auth::user::{
        kernel::data::AuthUserId,
        login_id::kernel::data::{LoginId, ValidateLoginIdError},
    },
    common::api::repository::data::RepositoryError,
};

pub struct UnregisterAuthUserAccountFields {
    pub login_id: LoginId,
}

pub trait UnregisterAuthUserAccountFieldsExtract {
    fn convert(self) -> Result<UnregisterAuthUserAccountFields, ValidateLoginIdError>;
}

#[async_trait::async_trait]
pub trait UnregisterAuthUserAccountRepository {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError>;

    async fn unregister_user(
        &self,
        user_id: &AuthUserId,
        login_id: &LoginId,
    ) -> Result<(), RepositoryError>;
}

#[async_trait::async_trait]
pub trait DiscardAuthTicketRepository {
    async fn discard_all(&self, user_id: &AuthUserId) -> Result<(), RepositoryError>;
}
