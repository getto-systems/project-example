use crate::{
    auth::user::{
        kernel::data::AuthUserId,
        login_id::{change::data::ValidateOverwriteLoginIdFieldsError, kernel::data::LoginId},
        password::reset::kernel::data::ResetPasswordTokenDestination,
    },
    common::api::repository::data::RepositoryError,
};

pub struct OverwriteLoginIdFields {
    pub login_id: LoginId,
    pub new_login_id: LoginId,
}

pub trait OverwriteLoginIdFieldsExtract {
    fn convert(self) -> Result<OverwriteLoginIdFields, ValidateOverwriteLoginIdFieldsError>;
}

#[async_trait::async_trait]
pub trait OverwriteLoginIdRepository {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<OverwriteLoginIdEntry>, RepositoryError>;

    async fn check_login_id_registered(&self, login_id: &LoginId) -> Result<bool, RepositoryError>;

    async fn overwrite_login_id(
        &self,
        // TODO この引数なんかおかしい
        new_login_id: LoginId,
        user: OverwriteLoginIdEntry,
    ) -> Result<(), RepositoryError>;
}

pub struct OverwriteLoginIdEntry {
    pub user_id: AuthUserId,
    pub login_id: LoginId,
    pub reset_token_destination: Option<ResetPasswordTokenDestination>,
}
