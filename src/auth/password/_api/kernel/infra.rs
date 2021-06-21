pub mod password_hasher;
pub mod password_matcher;
pub mod password_repository;
pub mod token_generator;

use super::convert::validate_password;

use super::data::{PasswordHashError, ResetToken, ValidatePasswordError};
use crate::z_details::_api::repository::data::RegisterAttemptResult;
use crate::{
    auth::{
        auth_ticket::_api::kernel::data::{AuthDateTime, ExpireDateTime},
        auth_user::_api::kernel::data::AuthUserId,
        login_id::_api::data::LoginId,
    },
    z_details::_api::repository::data::RepositoryError,
};

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

    pub fn extract(self) -> String {
        self.0
    }
}

pub trait AuthUserPasswordRepository {
    fn verify_password(
        &self,
        login_id: &LoginId,
        matcher: impl AuthUserPasswordMatcher,
    ) -> Result<Option<AuthUserId>, VerifyPasswordError>;

    fn register_reset_token(
        &self,
        user_id: AuthUserId,
        login_id: LoginId,
        token: ResetToken,
        expires: ExpireDateTime,
        registered_at: AuthDateTime,
    ) -> Result<RegisterAttemptResult<ResetToken>, RepositoryError>;
}

pub trait AuthUserPasswordMatcher {
    fn new(plain_password: PlainPassword) -> Self;
    fn match_password(&self, hashed_password: &HashedPassword) -> Result<bool, PasswordHashError>;
}

pub trait AuthUserPasswordHasher {
    fn hash_password(
        &self,
        plain_password: PlainPassword,
    ) -> Result<HashedPassword, PasswordHashError>;
}

pub trait ResetTokenGenerator {
    fn generate(&self) -> ResetToken;
}

pub enum VerifyPasswordError {
    PasswordMatchError(PasswordHashError),
    RepositoryError(RepositoryError),
}
