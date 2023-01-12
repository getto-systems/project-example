use crate::auth::user::password::kernel::infra::{HashedPassword, PlainPassword};

use crate::{
    auth::{
        ticket::kernel::data::AuthPermissionGranted,
        user::{
            kernel::data::AuthUserId, login_id::kernel::data::LoginId,
            password::authenticate::data::ValidateAuthenticateWithPasswordFieldsError,
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct AuthenticateWithPasswordFields {
    pub login_id: LoginId,
    pub plain_password: PlainPassword,
}

pub trait AuthenticateWithPasswordFieldsExtract {
    fn convert(
        self,
    ) -> Result<AuthenticateWithPasswordFields, ValidateAuthenticateWithPasswordFieldsError>;
}

#[async_trait::async_trait]
pub trait AuthenticatePasswordRepository {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, HashedPassword, Option<AuthPermissionGranted>)>, RepositoryError>;
}
