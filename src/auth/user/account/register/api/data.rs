use crate::{
    auth::user::account::kernel::data::ValidateAuthUserAccountError,
    common::api::repository::data::RepositoryError,
};

#[derive(Debug)]
pub struct RegisterAuthUserAccountSuccess;

#[derive(Debug)]
pub enum RegisterAuthUserAccountError {
    Invalid(ValidateAuthUserAccountError),
    LoginIdAlreadyRegistered,
    RepositoryError(RepositoryError),
}

const ERROR: &'static str = "register auth user account error";

impl std::fmt::Display for RegisterAuthUserAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
            Self::LoginIdAlreadyRegistered => {
                write!(f, "{}; login-id already registered", ERROR)
            }
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<ValidateAuthUserAccountError> for RegisterAuthUserAccountError {
    fn from(value: ValidateAuthUserAccountError) -> Self {
        Self::Invalid(value)
    }
}

impl From<RepositoryError> for RegisterAuthUserAccountError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
