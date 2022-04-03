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

pub enum ValidateOverridePasswordFieldsError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidPassword(ValidatePasswordError),
}

impl std::fmt::Display for ValidateOverridePasswordFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => write!(f, "login-id: {}", err),
            Self::InvalidPassword(err) => write!(f, "password: {}", err),
        }
    }
}
