use crate::auth::user::password::kernel::infra::{HashedPassword, PlainPassword};

use crate::{
    auth::user::{kernel::data::AuthUserId, login_id::kernel::data::LoginId},
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

pub trait OverwritePasswordRequestDecoder {
    fn decode(self) -> OverwritePasswordFieldsExtract;
}

pub struct OverwritePasswordFields {
    pub login_id: LoginId,
    pub new_password: PlainPassword,
}

pub struct OverwritePasswordFieldsExtract {
    pub login_id: String,
    pub new_password: String,
}

#[async_trait::async_trait]
pub trait ChangePasswordRepository {
    async fn lookup_password(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError>;

    async fn change_password(
        &self,
        user_id: AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError>;
}

#[async_trait::async_trait]
pub trait OverwritePasswordRepository {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError>;

    async fn overwrite_password(
        &self,
        user_id: AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError>;
}
