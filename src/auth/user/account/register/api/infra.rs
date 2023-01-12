use crate::{
    auth::user::{
        account::kernel::data::{AuthUserAccount, ValidateAuthUserAccountError},
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
    },
    common::api::repository::data::RepositoryError,
};

pub trait RegisterAuthUserAccountFieldsExtract {
    fn convert(self) -> Result<AuthUserAccount, ValidateAuthUserAccountError>;
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
