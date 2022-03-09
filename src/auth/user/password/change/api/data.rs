use crate::{
    auth::user::{
        login_id::kernel::data::ValidateLoginIdError,
        password::kernel::data::{PasswordHashError, ValidatePasswordError},
    },
    z_lib::repository::data::RepositoryError,
};

pub enum ChangePasswordError {
    InvalidCurrentPassword(ValidatePasswordError),
    InvalidNewPassword(ValidatePasswordError),
    PasswordNotFound,
    PasswordNotMatched,
}

impl std::fmt::Display for ChangePasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidCurrentPassword(err) => write!(f, "invalid current password: {}", err),
            Self::InvalidNewPassword(err) => write!(f, "invalid new password: {}", err),
            Self::PasswordNotFound => write!(f, "password not found"),
            Self::PasswordNotMatched => write!(f, "password not matched"),
        }
    }
}

pub enum ChangePasswordRepositoryError {
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
    PasswordNotFound,
    PasswordNotMatched,
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
            Self::InvalidPassword(err) => write!(f, "invalid new password: {}", err),
            Self::UserNotFound => write!(f, "user not found"),
        }
    }
}

pub enum OverridePasswordRepositoryError {
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
    UserNotFound,
}
