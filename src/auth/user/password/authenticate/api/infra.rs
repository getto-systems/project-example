use crate::{
    auth::user::{
        kernel::data::{AuthUserId, GrantedAuthRoles},
        login_id::kernel::data::LoginId,
        password::kernel::infra::{HashedPassword, PlainPassword},
    },
    z_lib::repository::data::RepositoryError,
};

pub trait AuthenticatePasswordRequestDecoder {
    fn decode(self) -> AuthenticatePasswordFieldsExtract;
}

pub struct AuthenticatePasswordFields {
    pub login_id: LoginId,
    pub password: PlainPassword,
}

pub struct AuthenticatePasswordFieldsExtract {
    pub login_id: String,
    pub password: String,
}

#[async_trait::async_trait]
pub trait AuthenticatePasswordRepository {
    async fn lookup_user_id<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError>;

    async fn lookup_granted_roles<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<GrantedAuthRoles>, RepositoryError>;

    async fn lookup_password<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError>;
}
