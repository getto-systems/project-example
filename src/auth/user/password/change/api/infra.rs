use crate::auth::user::password::kernel::infra::{AuthUserPasswordHasher, AuthUserPasswordMatcher};

use crate::auth::user::{
    kernel::data::AuthUserId,
    login_id::kernel::data::LoginId,
    password::change::data::{ChangePasswordRepositoryError, OverridePasswordRepositoryError},
};

pub trait ChangePasswordRequestDecoder {
    fn decode(self) -> ChangePasswordFieldsExtract;
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
    async fn change_password<'a>(
        &self,
        user_id: &'a AuthUserId,
        matcher: impl 'a + AuthUserPasswordMatcher,
        hasher: impl 'a + AuthUserPasswordHasher,
    ) -> Result<(), ChangePasswordRepositoryError>;
}

#[async_trait::async_trait]
pub trait OverridePasswordRepository {
    async fn override_password<'a>(
        &self,
        login_id: &'a LoginId,
        hasher: impl 'a + AuthUserPasswordHasher,
    ) -> Result<(), OverridePasswordRepositoryError>;
}
