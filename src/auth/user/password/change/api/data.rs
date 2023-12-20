use crate::{
    auth::{
        ticket::authorize::data::AuthorizeError,
        user::{
            login_id::kernel::data::ValidateLoginIdError,
            password::kernel::data::{PasswordHashError, ValidatePasswordError},
        },
    },
    common::api::repository::data::RepositoryError,
};

#[derive(Debug)]
pub struct ChangePasswordSuccess;

#[derive(Debug)]
pub enum ChangePasswordError {
    Invalid(ValidateChangePasswordFieldsError),
    NotFound,
    PasswordNotMatched,
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
    AuthorizeError(AuthorizeError),
}

mod change_password_error {
    const ERROR: &'static str = "change password error";

    impl std::fmt::Display for super::ChangePasswordError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
                Self::NotFound => write!(f, "{}; not found", ERROR),
                Self::PasswordNotMatched => write!(f, "{}; password not matched", ERROR),
                Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
                Self::AuthorizeError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

impl From<ValidateChangePasswordFieldsError> for ChangePasswordError {
    fn from(value: ValidateChangePasswordFieldsError) -> Self {
        Self::Invalid(value)
    }
}

impl From<PasswordHashError> for ChangePasswordError {
    fn from(value: PasswordHashError) -> Self {
        Self::PasswordHashError(value)
    }
}

impl From<RepositoryError> for ChangePasswordError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

impl From<AuthorizeError> for ChangePasswordError {
    fn from(value: AuthorizeError) -> Self {
        Self::AuthorizeError(value)
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct OverwritePasswordSuccess;

#[derive(Debug)]
pub enum OverwritePasswordError {
    Invalid(ValidateOverwritePasswordFieldsError),
    NotFound,
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
    AuthorizeError(AuthorizeError),
}

mod overwrite_password_error {
    const ERROR: &'static str = "overwrite password error";

    impl std::fmt::Display for super::OverwritePasswordError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
                Self::NotFound => write!(f, "{}; not found", ERROR),
                Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
                Self::AuthorizeError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

impl From<ValidateOverwritePasswordFieldsError> for OverwritePasswordError {
    fn from(value: ValidateOverwritePasswordFieldsError) -> Self {
        Self::Invalid(value)
    }
}

impl From<PasswordHashError> for OverwritePasswordError {
    fn from(value: PasswordHashError) -> Self {
        Self::PasswordHashError(value)
    }
}

impl From<RepositoryError> for OverwritePasswordError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

impl From<AuthorizeError> for OverwritePasswordError {
    fn from(value: AuthorizeError) -> Self {
        Self::AuthorizeError(value)
    }
}

#[derive(Debug)]
pub enum ValidateOverwritePasswordFieldsError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidNewPassword(ValidatePasswordError),
}

impl std::fmt::Display for ValidateOverwritePasswordFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => err.fmt(f),
            Self::InvalidNewPassword(err) => write!(f, "new-password: {}", err),
        }
    }
}
