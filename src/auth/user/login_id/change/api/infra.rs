use crate::{
    auth::user::{
        kernel::data::AuthUserId,
        login_id::{
            change::data::{
                OverwriteLoginIdError, OverwriteLoginIdSuccess, ValidateOverwriteLoginIdFieldsError,
            },
            kernel::data::LoginId,
        },
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

impl OverwriteLoginIdFieldsExtract for OverwriteLoginIdFields {
    fn convert(self) -> Result<OverwriteLoginIdFields, ValidateOverwriteLoginIdFieldsError> {
        Ok(self)
    }
}

pub trait OverwriteLoginIdInfra {
    type Repository: OverwriteLoginIdRepository;

    fn repository(&self) -> &Self::Repository;
}

#[async_trait::async_trait]
pub trait OverwriteLoginIdRepository {
    async fn check_login_id_registered(&self, login_id: &LoginId) -> Result<bool, RepositoryError>;

    async fn drop_login_id_entry(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, ResetPasswordTokenDestination)>, RepositoryError>;
    async fn insert_login_id_entry(
        &self,
        login_id: LoginId,
        user_id: AuthUserId,
        reset_token_destination: ResetPasswordTokenDestination,
    ) -> Result<(), RepositoryError>;
    async fn update_login_id(
        &self,
        user_id: AuthUserId,
        login_id: LoginId,
    ) -> Result<(), RepositoryError>;
}

pub trait OverwriteLoginIdLogger: Send + Sync {
    fn try_to_overwrite_login_id(&self);
    fn invalid_request(
        &self,
        err: ValidateOverwriteLoginIdFieldsError,
    ) -> ValidateOverwriteLoginIdFieldsError;
    fn failed_to_check_login_id_registered(&self, err: RepositoryError) -> RepositoryError;
    fn login_id_already_registered(&self, err: OverwriteLoginIdError) -> OverwriteLoginIdError;
    fn failed_to_drop_login_id_entry(&self, err: RepositoryError) -> RepositoryError;
    fn login_id_entry_not_found(&self, err: OverwriteLoginIdError) -> OverwriteLoginIdError;
    fn failed_to_insert_login_id_entry(&self, err: RepositoryError) -> RepositoryError;
    fn failed_to_update_login_id(&self, err: RepositoryError) -> RepositoryError;
    fn succeed_to_overwrite_login_id(
        &self,
        auth: OverwriteLoginIdSuccess,
    ) -> OverwriteLoginIdSuccess;
}
