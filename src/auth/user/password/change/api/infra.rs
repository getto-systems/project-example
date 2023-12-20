use crate::auth::user::password::kernel::infra::{
    AuthUserPasswordHasher, AuthUserPasswordMatcher, HashedPassword, PlainPassword,
};

use crate::{
    auth::user::{
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
        password::{
            change::data::{
                ChangePasswordError, ChangePasswordSuccess, OverwritePasswordError,
                OverwritePasswordSuccess, ValidateChangePasswordFieldsError,
                ValidateOverwritePasswordFieldsError,
            },
            kernel::data::PasswordHashError,
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct ChangePasswordFields {
    pub current_password: PlainPassword,
    pub new_password: PlainPassword,
}

pub trait ChangePasswordFieldsExtract {
    fn convert(self) -> Result<ChangePasswordFields, ValidateChangePasswordFieldsError>;
}

impl ChangePasswordFieldsExtract for ChangePasswordFields {
    fn convert(self) -> Result<ChangePasswordFields, ValidateChangePasswordFieldsError> {
        Ok(self)
    }
}

pub trait ChangePasswordInfra {
    type Repository: ChangePasswordRepository;
    type PasswordMatcher: AuthUserPasswordMatcher;
    type PasswordHasher: AuthUserPasswordHasher;

    fn repository(&self) -> &Self::Repository;
    fn password_matcher(&self, plain_password: PlainPassword) -> Self::PasswordMatcher {
        Self::PasswordMatcher::new(plain_password)
    }
    fn password_hasher(&self, plain_password: PlainPassword) -> Self::PasswordHasher {
        Self::PasswordHasher::new(plain_password)
    }
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

pub trait ChangePasswordLogger: Send + Sync {
    fn try_to_change_password(&self);
    fn invalid_request(
        &self,
        err: ValidateChangePasswordFieldsError,
    ) -> ValidateChangePasswordFieldsError;
    fn failed_to_lookup_password(&self, err: RepositoryError) -> RepositoryError;
    fn password_not_found(&self, err: ChangePasswordError) -> ChangePasswordError;
    fn failed_to_match_password(&self, err: PasswordHashError) -> PasswordHashError;
    fn password_not_matched(&self, err: ChangePasswordError) -> ChangePasswordError;
    fn failed_to_hash_password(&self, err: PasswordHashError) -> PasswordHashError;
    fn failed_to_change_password(&self, err: RepositoryError) -> RepositoryError;
    fn succeed_to_change_password(&self, auth: ChangePasswordSuccess) -> ChangePasswordSuccess;
}

pub struct OverwritePasswordFields {
    pub login_id: LoginId,
    pub new_password: PlainPassword,
}

pub trait OverwritePasswordFieldsExtract {
    fn convert(self) -> Result<OverwritePasswordFields, ValidateOverwritePasswordFieldsError>;
}

impl OverwritePasswordFieldsExtract for OverwritePasswordFields {
    fn convert(self) -> Result<OverwritePasswordFields, ValidateOverwritePasswordFieldsError> {
        Ok(self)
    }
}

pub trait OverwritePasswordInfra {
    type Repository: OverwritePasswordRepository;
    type PasswordHasher: AuthUserPasswordHasher;

    fn repository(&self) -> &Self::Repository;
    fn password_hasher(&self, plain_password: PlainPassword) -> Self::PasswordHasher {
        Self::PasswordHasher::new(plain_password)
    }
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

pub trait OverwritePasswordLogger: Send + Sync {
    fn try_to_overwrite_password(&self);
    fn invalid_request(
        &self,
        err: ValidateOverwritePasswordFieldsError,
    ) -> ValidateOverwritePasswordFieldsError;
    fn failed_to_lookup_user_id(&self, err: RepositoryError) -> RepositoryError;
    fn user_id_not_found(&self, err: OverwritePasswordError) -> OverwritePasswordError;
    fn failed_to_hash_password(&self, err: PasswordHashError) -> PasswordHashError;
    fn failed_to_overwrite_password(&self, err: RepositoryError) -> RepositoryError;
    fn succeed_to_overwrite_password(
        &self,
        auth: OverwritePasswordSuccess,
    ) -> OverwritePasswordSuccess;
}
