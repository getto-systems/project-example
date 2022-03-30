use crate::{
    auth::user::{
        account::modify::data::{ModifyAuthUserAccountData, ValidateAuthUserAccountError},
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
    pub from: ModifyAuthUserAccountData,
    pub to: ModifyAuthUserAccountData,
}

#[async_trait::async_trait]
pub trait ModifyAuthUserAccountRepository {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, ModifyAuthUserAccountData)>, RepositoryError>;

    async fn modify_user(
        &self,
        user_id: &AuthUserId,
        login_id: &LoginId,
        data: ModifyAuthUserAccountData,
    ) -> Result<(), RepositoryError>;

    async fn get_updated_user(
        &self,
        user_id: &AuthUserId,
        login_id: &LoginId,
    ) -> Result<ModifyAuthUserAccountData, RepositoryError>;
}
