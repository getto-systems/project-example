use chrono::{DateTime, Utc};

use crate::{
    auth::{
        ticket::remote::kernel::data::{AuthDateTime, ExpireDateTime},
        user::{
            login_id::remote::data::LoginId,
            password::remote::kernel::data::{
                ChangePasswordRepositoryError, PasswordHashError,
                RegisterResetTokenRepositoryError, ResetPasswordRepositoryError, ResetToken,
                ValidatePasswordError, VerifyPasswordRepositoryError,
            },
            remote::kernel::data::AuthUserId,
        },
    },
    z_lib::remote::repository::data::RepositoryError,
};

pub struct HashedPassword(String);

impl HashedPassword {
    pub(in crate::auth) const fn restore(password: String) -> Self {
        Self(password)
    }

    pub fn extract(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub struct PlainPassword(String);

impl PlainPassword {
    pub fn validate(
        password: impl PlainPasswordExtract,
    ) -> Result<PlainPassword, ValidatePasswordError> {
        Ok(Self(password.validate()?))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    #[cfg(test)]
    pub fn extract(self) -> String {
        self.0
    }
}

pub trait PlainPasswordExtract {
    fn validate(self) -> Result<String, ValidatePasswordError>;
}

#[async_trait::async_trait]
pub trait VerifyPasswordRepository {
    async fn verify_password<'a>(
        &self,
        login_id: &'a LoginId,
        matcher: impl AuthUserPasswordMatcher + 'a,
    ) -> Result<AuthUserId, VerifyPasswordRepositoryError>;
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
pub trait RegisterResetTokenRepository {
    async fn register_reset_token(
        &self,
        login_id: LoginId,
        reset_token: ResetToken,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) -> Result<(), RegisterResetTokenRepositoryError>;
}

#[async_trait::async_trait]
pub trait ResetPasswordRepository {
    async fn reset_token_entry(
        &self,
        reset_token: &ResetToken,
    ) -> Result<Option<ResetTokenEntry>, RepositoryError>;

    async fn reset_password<'a>(
        &self,
        reset_token: &'a ResetToken,
        hasher: impl AuthUserPasswordHasher + 'a,
        reset_at: AuthDateTime,
    ) -> Result<AuthUserId, ResetPasswordRepositoryError>;
}

pub struct ResetTokenEntry {
    login_id: LoginId,
    expires: ExpireDateTime,
    reset_at: Option<AuthDateTime>,
}

impl ResetTokenEntry {
    pub fn verify_login_id(&self, login_id: &LoginId) -> bool {
        self.login_id.as_str() == login_id.as_str()
    }

    pub fn has_expired(&self, now: &AuthDateTime) -> bool {
        self.expires.has_elapsed(now)
    }

    pub fn has_already_reset(&self) -> bool {
        self.reset_at.is_some()
    }
}

pub struct ResetTokenEntryExtract {
    pub login_id: String,
    pub expires: DateTime<Utc>,
    pub reset_at: Option<DateTime<Utc>>,
}

impl ResetTokenEntryExtract {
    pub(in crate::auth) fn restore(self) -> ResetTokenEntry {
        ResetTokenEntry {
            login_id: LoginId::restore(self.login_id),
            expires: ExpireDateTime::restore(self.expires),
            reset_at: self.reset_at.map(AuthDateTime::restore),
        }
    }
}

pub trait AuthUserPasswordMatcher: Send {
    fn new(plain_password: PlainPassword) -> Self;
    fn match_password(self, hashed_password: &HashedPassword) -> Result<bool, PasswordHashError>;
}

pub trait AuthUserPasswordHasher: Send {
    fn new(plain_password: PlainPassword) -> Self;
    fn hash_password(self) -> Result<HashedPassword, PasswordHashError>;
}
