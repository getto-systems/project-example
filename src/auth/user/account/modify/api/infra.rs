use crate::{
    auth::user::{
        account::modify::data::{
            ModifyAuthUserAccountChanges, ValidateModifyAuthUserAccountFieldsError,
        },
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
    },
    z_lib::repository::data::RepositoryError,
};

pub trait ModifyAuthUserAccountRequestDecoder {
    fn decode(
        self,
    ) -> Result<ModifyAuthUserAccountFields, ValidateModifyAuthUserAccountFieldsError>;
}

pub struct ModifyAuthUserAccountFields {
    pub login_id: LoginId,
    pub from: ModifyAuthUserAccountChanges,
    pub to: ModifyAuthUserAccountChanges,
}

#[async_trait::async_trait]
pub trait ModifyAuthUserAccountRepository {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError>;

    async fn lookup_changes(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<ModifyAuthUserAccountChanges>, RepositoryError>;

    async fn modify_user(
        &self,
        user_id: AuthUserId,
        data: ModifyAuthUserAccountChanges,
    ) -> Result<(), RepositoryError>;
}
