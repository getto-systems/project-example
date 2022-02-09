use crate::{
    auth::user::{
        login_id::kernel::data::ValidateLoginIdError,
        password::kernel::data::{PasswordHashError, ValidatePasswordError},
    },
    z_lib::repository::data::RepositoryError,
};

pub enum AuthenticatePasswordError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidPassword(ValidatePasswordError),
    UserNotFound,
    PasswordNotFound,
    PasswordNotMatched,
}

impl std::fmt::Display for AuthenticatePasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => write!(f, "invalid login id: {}", err),
            Self::InvalidPassword(err) => write!(f, "invalid password: {}", err),
            Self::UserNotFound => write!(f, "user not found"),
            Self::PasswordNotFound => write!(f, "password not found"),
            Self::PasswordNotMatched => write!(f, "password not matched"),
        }
    }
}

pub enum VerifyPasswordRepositoryError {
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
    UserNotFound,
    PasswordNotFound,
    PasswordNotMatched,
}
