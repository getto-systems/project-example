use crate::{
    auth::user::{
        kernel::data::AuthUserId, login_id::kernel::data::LoginId,
        password::reset::kernel::data::ResetTokenDestination,
    },
    z_lib::repository::data::RepositoryError,
};

pub struct OverwriteLoginIdFields {
    pub login_id: LoginId,
    pub new_login_id: LoginId,
}

pub struct OverwriteLoginIdFieldsExtract {
    pub login_id: String,
    pub new_login_id: String,
}

pub trait OverwriteLoginIdRequestDecoder {
    fn decode(self) -> OverwriteLoginIdFieldsExtract;
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
        new_login_id: LoginId,
        user: OverwriteLoginIdEntry,
    ) -> Result<(), RepositoryError>;
}

pub struct OverwriteLoginIdEntry {
    pub user_id: AuthUserId,
    pub login_id: LoginId,
    pub reset_token_destination: Option<ResetTokenDestination>,
}
