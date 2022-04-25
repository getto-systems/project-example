use crate::auth::user::{
    login_id::kernel::data::ValidateLoginIdError,
    password::reset::kernel::data::ValidateResetTokenDestinationError,
};

pub enum ValidateChangeResetTokenDestinationFieldsError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidFrom(ValidateChangeResetTokenDestinationChangesError),
    InvalidTo(ValidateChangeResetTokenDestinationChangesError),
}

pub enum ValidateChangeResetTokenDestinationChangesError {
    NotFound,
    InvalidResetTokenDestination(ValidateResetTokenDestinationError),
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

impl std::fmt::Display for ValidateChangeResetTokenDestinationChangesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotFound => write!(f, "data not found"),
            Self::InvalidResetTokenDestination(err) => err.fmt(f),
        }
    }
}
