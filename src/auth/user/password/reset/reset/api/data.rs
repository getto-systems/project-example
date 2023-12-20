use crate::{
    auth::user::password::{
        kernel::data::{PasswordHashError, ValidatePasswordError},
        reset::kernel::data::ValidateResetPasswordTokenError,
    },
    common::api::{notification::data::NotificationError, repository::data::RepositoryError},
};

#[derive(Debug)]
pub enum ResetPasswordError {
    Invalid(ValidateResetPasswordFieldsError),
    NotFound,
    ResetTokenExpired,
    AlreadyReset,
    RepositoryError(RepositoryError),
    PasswordHashError(PasswordHashError),
    DecodeError(DecodeResetTokenError),
    NotifyError(NotifyResetPasswordError),
}

const ERROR: &'static str = "reset password error";

impl std::fmt::Display for ResetPasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
            Self::NotFound => write!(f, "{}; not found", ERROR),
            Self::ResetTokenExpired => write!(f, "{}; reset token expired", ERROR),
            Self::AlreadyReset => write!(f, "{}; already reset", ERROR),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
            Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
            Self::NotifyError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<ValidateResetPasswordFieldsError> for ResetPasswordError {
    fn from(value: ValidateResetPasswordFieldsError) -> Self {
        Self::Invalid(value)
    }
}

impl From<RepositoryError> for ResetPasswordError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

impl From<PasswordHashError> for ResetPasswordError {
    fn from(value: PasswordHashError) -> Self {
        Self::PasswordHashError(value)
    }
}

impl From<DecodeResetTokenError> for ResetPasswordError {
    fn from(value: DecodeResetTokenError) -> Self {
        Self::DecodeError(value)
    }
}

impl From<NotifyResetPasswordError> for ResetPasswordError {
    fn from(value: NotifyResetPasswordError) -> Self {
        Self::NotifyError(value)
    }
}

#[derive(Debug)]
pub enum ValidateResetPasswordFieldsError {
    InvalidResetToken(ValidateResetPasswordTokenError),
    InvalidNewPassword(ValidatePasswordError),
}

impl std::fmt::Display for ValidateResetPasswordFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidResetToken(err) => write!(f, "reset-token: {}", err),
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

#[derive(Debug)]
pub enum NotifyResetPasswordError {
    NotificationError(NotificationError),
}

impl std::fmt::Display for NotifyResetPasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotificationError(err) => write!(f, "notify reset token error; {}", err),
        }
    }
}

impl From<NotificationError> for NotifyResetPasswordError {
    fn from(value: NotificationError) -> Self {
        Self::NotificationError(value)
    }
}

#[derive(Debug, Clone)]
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
