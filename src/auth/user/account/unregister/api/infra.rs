use crate::{
    auth::user::{
        account::unregister::data::{
            UnregisterAuthUserAccountError, UnregisterAuthUserAccountSuccess,
        },
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

impl UnregisterAuthUserAccountFieldsExtract for UnregisterAuthUserAccountFields {
    fn convert(self) -> Result<UnregisterAuthUserAccountFields, ValidateLoginIdError> {
        Ok(self)
    }
}

pub trait UnregisterAuthUserAccountInfra {
    type Repository: UnregisterAuthUserAccountRepository;

    fn repository(&self) -> &Self::Repository;
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

    async fn discard_all_ticket(&self, user_id: &AuthUserId) -> Result<(), RepositoryError>;
}

#[async_trait::async_trait]
pub trait DiscardAuthTicketRepository {
    async fn discard_all(&self, user_id: &AuthUserId) -> Result<(), RepositoryError>;
}

pub trait UnregisterAuthUserAccountLogger: Send + Sync {
    fn try_to_unregister_auth_user_account(&self);
    fn invalid_request(&self, err: ValidateLoginIdError) -> ValidateLoginIdError;
    fn failed_to_lookup_user_id(&self, err: RepositoryError) -> RepositoryError;
    fn user_id_not_found(
        &self,
        err: UnregisterAuthUserAccountError,
    ) -> UnregisterAuthUserAccountError;
    fn failed_to_unregister_user(&self, err: RepositoryError) -> RepositoryError;
    fn failed_to_discard_all_ticket(&self, err: RepositoryError) -> RepositoryError;
    fn succeed_to_unregister_auth_user_account(
        &self,
        success: UnregisterAuthUserAccountSuccess,
    ) -> UnregisterAuthUserAccountSuccess;
}
