use crate::{
    auth::user::{kernel::data::AuthUserId, login_id::kernel::data::LoginId},
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
    type User: Into<AuthUserId>;

    async fn lookup_user<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<Option<Self::User>, RepositoryError>;

    async fn check_login_id_registered<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<bool, RepositoryError>;

    async fn override_login_id<'a>(
        &self,
        user: Self::User,
        new_login_id: LoginId,
    ) -> Result<(), RepositoryError>;
}
