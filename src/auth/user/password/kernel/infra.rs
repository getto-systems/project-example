use crate::{
    auth::user::password::kernel::data::{PasswordHashError, ValidatePasswordError},
    common::api::validate::data::ValidateTextError,
};

#[derive(Clone)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub(in crate::auth) const fn restore(password: String) -> Self {
        Self(password)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

pub struct PlainPassword(String);

impl PlainPassword {
    pub fn convert(
        value: impl PlainPasswordExtract,
    ) -> Result<PlainPassword, ValidatePasswordError> {
        Ok(Self(
            value.convert().map_err(ValidatePasswordError::Password)?,
        ))
    }

    #[cfg(test)]
    pub fn restore(value: String) -> Self {
        Self(value)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

pub trait PlainPasswordExtract {
    fn convert(self) -> Result<String, ValidateTextError>;
}

pub trait AuthUserPasswordMatcher: Send {
    fn new(plain_password: PlainPassword) -> Self;
    fn match_password(self, hashed_password: HashedPassword) -> Result<bool, PasswordHashError>;
}

pub trait AuthUserPasswordHasher: Send {
    fn new(plain_password: PlainPassword) -> Self;
    fn hash_password(self) -> Result<HashedPassword, PasswordHashError>;
}
