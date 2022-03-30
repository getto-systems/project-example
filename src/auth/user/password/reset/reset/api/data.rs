use crate::{
    auth::user::{
        login_id::kernel::data::ValidateLoginIdError,
        password::{
            kernel::data::{PasswordHashError, ValidatePasswordError},
            reset::kernel::data::ValidateResetTokenError,
        },
    },
    z_lib::repository::data::RepositoryError,
};

pub enum VerifyResetTokenEntryError {
    ResetTokenEntryNotFound,
    LoginIdNotMatched,
    Expired,
    AlreadyReset,
}

impl std::fmt::Display for VerifyResetTokenEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::ResetTokenEntryNotFound => write!(f, "reset token entry not found"),
            Self::LoginIdNotMatched => write!(f, "login id not matched"),
            Self::Expired => write!(f, "reset token expired"),
            Self::AlreadyReset => write!(f, "already reset"),
        }
    }
}

pub enum ResetPasswordRepositoryError {
    ResetTokenNotFound,
    RepositoryError(RepositoryError),
    PasswordHashError(PasswordHashError),
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
