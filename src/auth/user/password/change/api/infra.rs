use crate::auth::user::password::kernel::infra::{
    AuthUserPasswordHasher, HashedPassword, PlainPassword,
};

use crate::{
    auth::user::{
        kernel::data::AuthUserId, login_id::kernel::data::LoginId,
        password::change::data::OverridePasswordRepositoryError,
    },
    z_lib::repository::data::RepositoryError,
};

pub trait ChangePasswordRequestDecoder {
    fn decode(self) -> ChangePasswordFieldsExtract;
}

pub struct ChangePasswordFields {
    pub current_password: PlainPassword,
    pub new_password: PlainPassword,
}

pub struct ChangePasswordFieldsExtract {
    pub current_password: String,
    pub new_password: String,
}

pub trait OverridePasswordRequestDecoder {
    fn decode(self) -> OverridePasswordFieldsExtract;
}

pub struct OverridePasswordFieldsExtract {
    pub login_id: String,
    pub new_password: String,
}

#[async_trait::async_trait]
pub trait ChangePasswordRepository {
    async fn lookup_password<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError>;

    async fn change_password<'a>(
        &self,
        user_id: &'a AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError>;
}

#[async_trait::async_trait]
pub trait OverridePasswordRepository {
    async fn override_password<'a>(
        &self,
        login_id: &'a LoginId,
        hasher: impl 'a + AuthUserPasswordHasher,
    ) -> Result<(), OverridePasswordRepositoryError>;
}
