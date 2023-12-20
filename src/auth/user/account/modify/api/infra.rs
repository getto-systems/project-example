use crate::{
    auth::user::{
        account::{
            kernel::data::AuthUserAccountAttrs,
            modify::data::{
                ModifyAuthUserAccountError, ModifyAuthUserAccountSuccess,
                ValidateModifyAuthUserAccountFieldsError,
            },
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

impl ModifyAuthUserAccountFieldsExtract for ModifyAuthUserAccountFields {
    fn convert(
        self,
    ) -> Result<ModifyAuthUserAccountFields, ValidateModifyAuthUserAccountFieldsError> {
        Ok(self)
    }
}

pub trait ModifyAuthUserAccountInfra {
    type Repository: ModifyAuthUserAccountRepository;

    fn repository(&self) -> &Self::Repository;
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

pub trait ModifyAuthUserAccountLogger: Send + Sync {
    fn try_to_modify_auth_user_account(&self);
    fn invalid_request(
        &self,
        err: ValidateModifyAuthUserAccountFieldsError,
    ) -> ValidateModifyAuthUserAccountFieldsError;
    fn failed_to_lookup_user_id(&self, err: RepositoryError) -> RepositoryError;
    fn user_id_not_found(&self, err: ModifyAuthUserAccountError) -> ModifyAuthUserAccountError;
    fn failed_to_lookup_attrs(&self, err: RepositoryError) -> RepositoryError;
    fn user_attrs_not_found(&self, err: ModifyAuthUserAccountError) -> ModifyAuthUserAccountError;
    fn conflict(&self, err: ModifyAuthUserAccountError) -> ModifyAuthUserAccountError;
    fn failed_to_modify_attrs(&self, err: RepositoryError) -> RepositoryError;
    fn succeed_to_modify_auth_user_account(
        &self,
        success: ModifyAuthUserAccountSuccess,
    ) -> ModifyAuthUserAccountSuccess;
}
