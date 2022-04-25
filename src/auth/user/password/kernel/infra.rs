use crate::auth::user::password::kernel::data::{PasswordHashError, ValidatePasswordError};

#[derive(Clone)]
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
    pub fn convert(
        password: impl PlainPasswordExtract,
    ) -> Result<PlainPassword, ValidatePasswordError> {
        Ok(Self(password.convert()?))
    }

    pub fn extract(self) -> String {
        self.0
    }
}

pub trait PlainPasswordExtract {
    fn convert(self) -> Result<String, ValidatePasswordError>;
}

pub trait AuthUserPasswordMatcher: Send {
    fn new(plain_password: PlainPassword) -> Self;
    fn match_password(self, hashed_password: &HashedPassword) -> Result<bool, PasswordHashError>;
}

pub trait AuthUserPasswordHasher: Send {
    fn new(plain_password: PlainPassword) -> Self;
    fn hash_password(self) -> Result<HashedPassword, PasswordHashError>;
}
