use crate::{
    auth::user::{
        login_id::kernel::data::ValidateLoginIdError,
        password::reset::kernel::data::ValidateResetPasswordTokenDestinationError,
    },
    common::api::repository::data::RepositoryError,
};

#[derive(Debug)]
pub struct ChangeResetTokenDestinationSuccess;

#[derive(Debug)]
pub enum ChangeResetTokenDestinationError {
    Invalid(ValidateChangeResetTokenDestinationFieldsError),
    NotFound,
    Conflict,
    RepositoryError(RepositoryError),
}

const ERROR: &'static str = "change reset token destination error";

impl std::fmt::Display for ChangeResetTokenDestinationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
            Self::NotFound => write!(f, "{}; not found", ERROR),
            Self::Conflict => write!(f, "{}; changes conflicted", ERROR),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<ValidateChangeResetTokenDestinationFieldsError> for ChangeResetTokenDestinationError {
    fn from(value: ValidateChangeResetTokenDestinationFieldsError) -> Self {
        Self::Invalid(value)
    }
}

impl From<RepositoryError> for ChangeResetTokenDestinationError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

#[derive(Debug)]
pub enum ValidateChangeResetTokenDestinationFieldsError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidFrom(ValidateResetPasswordTokenDestinationError),
    InvalidTo(ValidateResetPasswordTokenDestinationError),
}

impl std::fmt::Display for ValidateChangeResetTokenDestinationFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => err.fmt(f),
            Self::InvalidFrom(err) => write!(f, "from: {}", err),
            Self::InvalidTo(err) => write!(f, "to: {}", err),
        }
    }
}
