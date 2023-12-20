use crate::auth::ticket::kernel::data::AuthenticateSuccess;
use crate::auth::user::password::kernel::infra::{
    AuthUserPasswordMatcher, HashedPassword, PlainPassword,
};

use crate::{
    auth::{
        ticket::kernel::data::AuthPermissionGranted,
        user::{
            kernel::data::AuthUserId,
            login_id::kernel::data::LoginId,
            password::{
                authenticate::data::{
                    AuthenticateWithPasswordError, ValidateAuthenticateWithPasswordFieldsError,
                },
                kernel::data::PasswordHashError,
            },
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

impl AuthenticateWithPasswordFieldsExtract for AuthenticateWithPasswordFields {
    fn convert(
        self,
    ) -> Result<AuthenticateWithPasswordFields, ValidateAuthenticateWithPasswordFieldsError> {
        Ok(self)
    }
}

pub trait AuthenticateWithPasswordInfra {
    type Repository: AuthenticateWithPasswordRepository;
    type PasswordMatcher: AuthUserPasswordMatcher;

    fn repository(&self) -> &Self::Repository;
    fn password_matcher(&self, plain_password: PlainPassword) -> Self::PasswordMatcher {
        Self::PasswordMatcher::new(plain_password)
    }
}

#[async_trait::async_trait]
pub trait AuthenticateWithPasswordRepository {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError>;
    async fn lookup_password_and_granted(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<(HashedPassword, Option<AuthPermissionGranted>)>, RepositoryError>;
}

pub trait AuthenticateWithPasswordLogger: Send + Sync {
    fn try_to_authenticate_with_password(&self);
    fn invalid_request(
        &self,
        err: ValidateAuthenticateWithPasswordFieldsError,
    ) -> ValidateAuthenticateWithPasswordFieldsError;
    fn failed_to_lookup_user(&self, err: RepositoryError) -> RepositoryError;
    fn failed_to_lookup_password_and_granted(&self, err: RepositoryError) -> RepositoryError;
    fn user_not_found(&self, err: AuthenticateWithPasswordError) -> AuthenticateWithPasswordError;
    fn failed_to_match_password(&self, err: PasswordHashError) -> PasswordHashError;
    fn password_not_matched(
        &self,
        err: AuthenticateWithPasswordError,
    ) -> AuthenticateWithPasswordError;
    fn authenticated(&self, auth: AuthenticateSuccess) -> AuthenticateSuccess;
}
