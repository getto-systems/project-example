use super::convert::validate_password;

use crate::auth::{
    auth_ticket::_api::kernel::data::{AuthDateTime, ExpireDateTime},
    auth_user::_api::kernel::data::AuthUserId,
    login_id::_api::data::LoginId,
    password::_api::kernel::data::{PasswordHashError, ResetToken, ValidatePasswordError},
};
use crate::z_details::_api::repository::data::RepositoryError;

pub trait AuthUserPasswordInfra {
    type PasswordRepository: AuthUserPasswordRepository;
    type PasswordMatcher: AuthUserPasswordMatcher;
    type PasswordHasher: AuthUserPasswordHasher;

    fn password_repository(&self) -> &Self::PasswordRepository;
    fn password_matcher(&self, plain_password: PlainPassword) -> Self::PasswordMatcher {
        Self::PasswordMatcher::new(plain_password)
    }
    fn password_hasher(&self, plain_password: PlainPassword) -> Self::PasswordHasher {
        Self::PasswordHasher::new(plain_password)
    }
}

pub struct HashedPassword(String);

impl HashedPassword {
    pub const fn new(password: String) -> Self {
        Self(password)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub struct PlainPassword(String);

impl PlainPassword {
    pub fn validate(password: String) -> Result<PlainPassword, ValidatePasswordError> {
        validate_password(&password)?;
        Ok(Self(password))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    #[cfg(test)]
    pub fn extract(self) -> String {
        self.0
    }
}

pub trait AuthUserPasswordRepository {
    fn verify_password(
        &self,
        login_id: &LoginId,
        matcher: &impl AuthUserPasswordMatcher,
    ) -> Result<AuthUserId, VerifyPasswordError>;

    fn register_reset_token(
        &self,
        reset_token: ResetToken,
        login_id: LoginId,
        expires: ExpireDateTime,
        registered_at: AuthDateTime,
    ) -> Result<(), RegisterResetTokenError>;

    fn reset_password(
        &self,
        reset_token: &ResetToken,
        login_id: &LoginId,
        hasher: &impl AuthUserPasswordHasher,
        reset_at: &AuthDateTime,
    ) -> Result<AuthUserId, ResetPasswordError>;
}

pub trait AuthUserPasswordMatcher {
    fn new(plain_password: PlainPassword) -> Self;
    fn match_password(&self, hashed_password: &HashedPassword) -> Result<bool, PasswordHashError>;
}

pub trait AuthUserPasswordHasher {
    fn new(plain_password: PlainPassword) -> Self;
    fn hash_password(&self) -> Result<HashedPassword, PasswordHashError>;
}

pub enum VerifyPasswordError {
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
    UserNotFound,
    PasswordNotFound,
    PasswordNotMatched,
}

pub enum RegisterResetTokenError {
    RepositoryError(RepositoryError),
    NotFound,
}

pub enum ResetPasswordError {
    RepositoryError(RepositoryError),
    PasswordHashError(PasswordHashError),
    NotFound,
    AlreadyReset,
    Expired,
    InvalidLoginId,
}
