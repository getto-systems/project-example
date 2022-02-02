use crate::{
    auth::user::{
        login_id::remote::data::ValidateLoginIdError,
        password::remote::kernel::data::{PasswordHashError, ValidatePasswordError},
    },
    z_lib::remote::repository::data::RepositoryError,
};

pub enum AuthenticatePasswordError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidPassword(ValidatePasswordError),
    PasswordNotFound,
    PasswordNotMatched,
}

impl std::fmt::Display for AuthenticatePasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => write!(f, "invalid login id: {}", err),
            Self::InvalidPassword(err) => write!(f, "invalid password: {}", err),
            Self::PasswordNotFound => write!(f, "password not found"),
            Self::PasswordNotMatched => write!(f, "password not matched"),
        }
    }
}

pub enum VerifyPasswordRepositoryError {
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
    PasswordNotFound,
    PasswordNotMatched,
}
