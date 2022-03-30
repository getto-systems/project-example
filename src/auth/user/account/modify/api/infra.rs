use crate::{
    auth::user::{
        account::modify::data::{AuthUserAccountChanges, ValidateAuthUserAccountError},
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
    },
    z_lib::repository::data::RepositoryError,
};

pub trait ModifyAuthUserAccountRequestDecoder {
    fn decode(self) -> Result<ModifyAuthUserAccountFields, ValidateAuthUserAccountError>;
}

pub struct ModifyAuthUserAccountFields {
    pub login_id: LoginId,
    pub from: AuthUserAccountChanges,
    pub to: AuthUserAccountChanges,
}

#[async_trait::async_trait]
pub trait ModifyAuthUserAccountRepository {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, AuthUserAccountChanges)>, RepositoryError>;

    async fn modify_user(
        &self,
        user_id: &AuthUserId,
        data: AuthUserAccountChanges,
    ) -> Result<(), RepositoryError>;

    async fn get_updated_user(
        &self,
        user_id: &AuthUserId,
    ) -> Result<AuthUserAccountChanges, RepositoryError>;
}
