use crate::{
    auth::user::login_id::kernel::data::ValidateLoginIdError,
    common::api::repository::data::RepositoryError,
};

#[derive(Debug)]
pub struct OverwriteLoginIdSuccess;

#[derive(Debug)]
pub enum OverwriteLoginIdError {
    Invalid(ValidateOverwriteLoginIdFieldsError),
    NotFound,
    AlreadyRegistered,
    RepositoryError(RepositoryError),
}

const ERROR: &'static str = "overwrite login-id error";

impl std::fmt::Display for OverwriteLoginIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
            Self::NotFound => write!(f, "{}; not found", ERROR),
            Self::AlreadyRegistered => {
                write!(f, "{}; new login id is already registered", ERROR)
            }
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<ValidateOverwriteLoginIdFieldsError> for OverwriteLoginIdError {
    fn from(value: ValidateOverwriteLoginIdFieldsError) -> Self {
        Self::Invalid(value)
    }
}

impl From<RepositoryError> for OverwriteLoginIdError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

#[derive(Debug)]
pub enum ValidateOverwriteLoginIdFieldsError {
    InvalidCurrentLoginId(ValidateLoginIdError),
    InvalidNewLoginId(ValidateLoginIdError),
}

impl std::fmt::Display for ValidateOverwriteLoginIdFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCurrentLoginId(err) => write!(f, "current: {}", err),
            Self::InvalidNewLoginId(err) => write!(f, "new: {}", err),
        }
    }
}
