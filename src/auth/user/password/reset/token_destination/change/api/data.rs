use crate::auth::user::{
    login_id::kernel::data::ValidateLoginIdError,
    password::reset::kernel::data::ValidateResetPasswordTokenDestinationError,
};

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
