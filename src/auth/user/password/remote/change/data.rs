use crate::auth::user::password::remote::kernel::data::ValidatePasswordError;

pub enum ChangePasswordError {
    InvalidCurrentPassword(ValidatePasswordError),
    InvalidNewPassword(ValidatePasswordError),
    PasswordNotFound,
    PasswordNotMatched,
}

impl std::fmt::Display for ChangePasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidCurrentPassword(err) => write!(f, "invalid current password: {}", err),
            Self::InvalidNewPassword(err) => write!(f, "invalid new password: {}", err),
            Self::PasswordNotFound => write!(f, "password not found"),
            Self::PasswordNotMatched => write!(f, "password not matched"),
        }
    }
}
