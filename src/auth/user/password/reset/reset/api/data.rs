use crate::auth::user::{
    login_id::kernel::data::ValidateLoginIdError,
    password::{kernel::data::ValidatePasswordError, reset::kernel::data::ValidateResetTokenError},
};

pub enum ValidateResetPasswordFieldsError {
    InvalidResetToken(ValidateResetTokenError),
    InvalidLoginId(ValidateLoginIdError),
    InvalidNewPassword(ValidatePasswordError),
}

impl std::fmt::Display for ValidateResetPasswordFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidResetToken(err) => write!(f, "reset-token: {}", err),
            Self::InvalidLoginId(err) => write!(f, "login-id: {}", err),
            Self::InvalidNewPassword(err) => write!(f, "new-password: {}", err),
        }
    }
}

pub enum NotifyResetPasswordResponse {
    NoDestination,
    Send(String),
}

impl std::fmt::Display for NotifyResetPasswordResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NoDestination => write!(f, "no destination"),
            Self::Send(message_id) => write!(f, "message-id: {}", message_id),
        }
    }
}

pub enum NotifyResetPasswordError {
    InfraError(String),
}

impl std::fmt::Display for NotifyResetPasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "notify reset token error; {}", err),
        }
    }
}

pub enum DecodeResetTokenError {
    Expired,
    Invalid(String),
}

impl std::fmt::Display for DecodeResetTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Expired => write!(f, "reset token expired"),
            Self::Invalid(err) => write!(f, "decode error; {}", err),
        }
    }
}
