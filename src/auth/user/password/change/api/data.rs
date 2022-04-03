use crate::{
    auth::user::{
        login_id::kernel::data::ValidateLoginIdError,
        password::kernel::data::{PasswordHashError, ValidatePasswordError},
    },
    z_lib::repository::data::RepositoryError,
};

pub enum ValidateChangePasswordFieldsError {
    InvalidCurrentPassword(ValidatePasswordError),
    InvalidNewPassword(ValidatePasswordError),
}

impl std::fmt::Display for ValidateChangePasswordFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidCurrentPassword(err) => write!(f, "current: {}", err),
            Self::InvalidNewPassword(err) => write!(f, "new: {}", err),
        }
    }
}

pub enum OverridePasswordError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidPassword(ValidatePasswordError),
    UserNotFound,
}

impl std::fmt::Display for OverridePasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => write!(f, "invalid login id: {}", err),
            Self::InvalidPassword(err) => write!(f, "invalid password: {}", err),
            Self::UserNotFound => write!(f, "user not found"),
        }
    }
}

pub enum OverridePasswordRepositoryError {
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
    UserNotFound,
}
