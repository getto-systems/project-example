use crate::{
    auth::user::{
        account::{
            kernel::data::{AuthUserAccount, ValidateAuthUserAccountError},
            register::data::{RegisterAuthUserAccountError, RegisterAuthUserAccountSuccess},
        },
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
    },
    common::api::repository::data::RepositoryError,
};

pub trait RegisterAuthUserAccountFieldsExtract {
    fn convert(self) -> Result<AuthUserAccount, ValidateAuthUserAccountError>;
}

impl RegisterAuthUserAccountFieldsExtract for AuthUserAccount {
    fn convert(self) -> Result<AuthUserAccount, ValidateAuthUserAccountError> {
        Ok(self)
    }
}

pub trait RegisterAuthUserAccountInfra {
    type UserIdGenerator: AuthUserIdGenerator;
    type Repository: RegisterAuthUserAccountRepository;

    fn user_id_generator(&self) -> &Self::UserIdGenerator;
    fn repository(&self) -> &Self::Repository;
}

pub trait AuthUserIdGenerator {
    fn generate(&self) -> AuthUserId;
}

#[async_trait::async_trait]
pub trait RegisterAuthUserAccountRepository {
    async fn check_login_id_registered(&self, login_id: &LoginId) -> Result<bool, RepositoryError>;

    async fn register_user(
        &self,
        user_id: AuthUserId,
        data: AuthUserAccount,
    ) -> Result<(), RepositoryError>;
}

pub trait RegisterAuthUserAccountLogger: Send + Sync {
    fn try_to_register_auth_user_account(&self);
    fn invalid_request(&self, err: ValidateAuthUserAccountError) -> ValidateAuthUserAccountError;
    fn failed_to_check_login_id_registered(&self, err: RepositoryError) -> RepositoryError;
    fn login_id_already_registered(
        &self,
        err: RegisterAuthUserAccountError,
    ) -> RegisterAuthUserAccountError;
    fn failed_to_register_user(&self, err: RepositoryError) -> RepositoryError;
    fn succeed_to_register_auth_user_account(
        &self,
        success: RegisterAuthUserAccountSuccess,
    ) -> RegisterAuthUserAccountSuccess;
}
