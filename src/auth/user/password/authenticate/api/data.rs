use crate::{
    auth::user::{
        login_id::kernel::data::{LoginId, ValidateLoginIdError},
        password::kernel::data::{PasswordHashError, ValidatePasswordError},
    },
    common::api::repository::data::RepositoryError,
};

#[derive(Debug)]
pub enum AuthenticateWithPasswordError {
    Invalid(ValidateAuthenticateWithPasswordFieldsError),
    NotFound(LoginId),
    PasswordNotMatched,
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
}

const ERROR: &'static str = "authenticate with password error";

impl std::fmt::Display for AuthenticateWithPasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
            Self::NotFound(login_id) => write!(f, "{}; not found; login-id: {}", ERROR, login_id),
            Self::PasswordNotMatched => write!(f, "{}; password not matched", ERROR),
            Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<ValidateAuthenticateWithPasswordFieldsError> for AuthenticateWithPasswordError {
    fn from(value: ValidateAuthenticateWithPasswordFieldsError) -> Self {
        Self::Invalid(value)
    }
}

impl From<PasswordHashError> for AuthenticateWithPasswordError {
    fn from(value: PasswordHashError) -> Self {
        Self::PasswordHashError(value)
    }
}

impl From<RepositoryError> for AuthenticateWithPasswordError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

#[derive(Debug)]
pub enum ValidateAuthenticateWithPasswordFieldsError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidPassword(ValidatePasswordError),
}

impl std::fmt::Display for ValidateAuthenticateWithPasswordFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => err.fmt(f),
            Self::InvalidPassword(err) => write!(f, "password: {}", err),
        }
    }
}
