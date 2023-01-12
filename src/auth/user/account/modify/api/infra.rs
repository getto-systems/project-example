use crate::{
    auth::user::{
        account::{
            kernel::data::AuthUserAccountAttrs,
            modify::data::ValidateModifyAuthUserAccountFieldsError,
        },
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
    },
    common::api::repository::data::RepositoryError,
};

pub struct ModifyAuthUserAccountFields {
    pub login_id: LoginId,
    pub from: AuthUserAccountAttrs,
    pub to: AuthUserAccountAttrs,
}

pub trait ModifyAuthUserAccountFieldsExtract {
    fn convert(
        self,
    ) -> Result<ModifyAuthUserAccountFields, ValidateModifyAuthUserAccountFieldsError>;
}

#[async_trait::async_trait]
pub trait ModifyAuthUserAccountRepository {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError>;

    async fn lookup_attrs(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<AuthUserAccountAttrs>, RepositoryError>;

    async fn modify_user(
        &self,
        user_id: AuthUserId,
        data: AuthUserAccountAttrs,
    ) -> Result<(), RepositoryError>;
}
