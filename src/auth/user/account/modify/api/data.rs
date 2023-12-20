use crate::{
    auth::user::{
        account::kernel::data::ValidateAuthUserAccountError,
        login_id::kernel::data::ValidateLoginIdError,
    },
    common::api::repository::data::RepositoryError,
};

#[derive(Debug)]
pub struct ModifyAuthUserAccountSuccess;

#[derive(Debug)]
pub enum ModifyAuthUserAccountError {
    Invalid(ValidateModifyAuthUserAccountFieldsError),
    NotFound,
    Conflict,
    RepositoryError(RepositoryError),
}

const ERROR: &'static str = "modify auth-user-account error";

impl std::fmt::Display for ModifyAuthUserAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}; {}", ERROR, err),
            Self::NotFound => write!(f, "{}; not found", ERROR),
            Self::Conflict => write!(f, "{}; changes conflicted", ERROR),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<ValidateModifyAuthUserAccountFieldsError> for ModifyAuthUserAccountError {
    fn from(value: ValidateModifyAuthUserAccountFieldsError) -> Self {
        Self::Invalid(value)
    }
}

impl From<RepositoryError> for ModifyAuthUserAccountError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

#[derive(Debug)]
pub enum ValidateModifyAuthUserAccountFieldsError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidFrom(ValidateAuthUserAccountError),
    InvalidTo(ValidateAuthUserAccountError),
}

impl std::fmt::Display for ValidateModifyAuthUserAccountFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => err.fmt(f),
            Self::InvalidFrom(err) => write!(f, "invalid from; {}", err),
            Self::InvalidTo(err) => write!(f, "invalid to; {}", err),
        }
    }
}
