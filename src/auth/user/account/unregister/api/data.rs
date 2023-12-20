use crate::{
    auth::user::login_id::kernel::data::ValidateLoginIdError,
    common::api::repository::data::RepositoryError,
};

#[derive(Debug)]
pub struct UnregisterAuthUserAccountSuccess;

#[derive(Debug)]
pub enum UnregisterAuthUserAccountError {
    Invalid(ValidateLoginIdError),
    NotFound,
    RepositoryError(RepositoryError),
}

const ERROR: &'static str = "unregister auth user account error";

impl std::fmt::Display for UnregisterAuthUserAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
            Self::NotFound => write!(f, "{}; user not found", ERROR),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<ValidateLoginIdError> for UnregisterAuthUserAccountError {
    fn from(value: ValidateLoginIdError) -> Self {
        Self::Invalid(value)
    }
}

impl From<RepositoryError> for UnregisterAuthUserAccountError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
