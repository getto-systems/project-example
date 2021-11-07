use crate::auth::user::{
    login_id::remote::data::ValidateLoginIdError,
    password::{
        remote::kernel::data::{ValidatePasswordError, VerifyResetTokenEntryError},
        reset::remote::kernel::data::ValidateResetTokenError,
    },
};

pub struct NotifyResetPasswordResponse {
    message_id: String,
}

impl NotifyResetPasswordResponse {
    pub fn new(message_id: String) -> Self {
        Self { message_id }
    }
}

impl std::fmt::Display for NotifyResetPasswordResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "message-id: {}", self.message_id)
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

pub enum ResetPasswordError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidPassword(ValidatePasswordError),
    InvalidResetToken(ValidateResetTokenError),
    InvalidResetTokenEntry(VerifyResetTokenEntryError),
}

impl std::fmt::Display for ResetPasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => write!(f, "invalid login id: {}", err),
            Self::InvalidPassword(err) => write!(f, "invalid password: {}", err),
            Self::InvalidResetToken(err) => write!(f, "invalid reset token: {}", err),
            Self::InvalidResetTokenEntry(err) => write!(f, "invalid reset token entry: {}", err),
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
