use crate::{
    auth::user::{
        kernel::data::AuthUserId, login_id::kernel::data::LoginId,
        password::reset::kernel::data::ResetTokenDestination,
    },
    z_lib::repository::data::RepositoryError,
};

pub struct OverrideLoginIdFields {
    pub login_id: LoginId,
    pub new_login_id: LoginId,
}

pub struct OverrideLoginIdFieldsExtract {
    pub login_id: String,
    pub new_login_id: String,
}

pub trait OverrideLoginIdRequestDecoder {
    fn decode(self) -> OverrideLoginIdFieldsExtract;
}

#[async_trait::async_trait]
pub trait OverrideLoginIdRepository {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<OverrideLoginIdEntry>, RepositoryError>;

    async fn check_login_id_registered(&self, login_id: &LoginId) -> Result<bool, RepositoryError>;

    async fn override_login_id(
        &self,
        new_login_id: LoginId,
        user: OverrideLoginIdEntry,
    ) -> Result<(), RepositoryError>;
}

pub struct OverrideLoginIdEntry {
    pub user_id: AuthUserId,
    pub login_id: LoginId,
    pub reset_token_destination: Option<ResetTokenDestination>,
}
