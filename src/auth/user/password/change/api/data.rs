use crate::auth::user::{
    login_id::kernel::data::ValidateLoginIdError, password::kernel::data::ValidatePasswordError,
};

pub enum ValidateChangePasswordFieldsError {
    InvalidCurrentPassword(ValidatePasswordError),
    InvalidNewPassword(ValidatePasswordError),
}

impl std::fmt::Display for ValidateChangePasswordFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidCurrentPassword(err) => write!(f, "current: {}", err),
            Self::InvalidNewPassword(err) => write!(f, "new: {}", err),
        }
    }
}

pub enum ValidateOverwritePasswordFieldsError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidNewPassword(ValidatePasswordError),
}

impl std::fmt::Display for ValidateOverwritePasswordFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => err.fmt(f),
            Self::InvalidNewPassword(err) => write!(f, "new-password: {}", err),
        }
    }
}
