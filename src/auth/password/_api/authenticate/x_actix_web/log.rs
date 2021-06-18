use crate::{auth::password::_api::authenticate::data::{AuthenticatePasswordResponse, ValidatePasswordError, PasswordMatchError}, x_outside_feature::_api::logger::LogLevel};

use super::super::event::AuthenticatePasswordEvent;

impl AuthenticatePasswordEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Audit,
            Self::UserNotFound => LogLevel::Error,
            Self::InvalidPassword(err) => err.log_level(),
            Self::NonceError(err) => err.log_level(),
            Self::PasswordHashError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
            Self::MessageError(err) => err.log_level(),
            Self::ValidateLoginIdError(err) => err.log_level(),
            Self::ValidatePasswordError(err) => err.log_level(),
        }
    }
}

impl AuthenticatePasswordResponse {
    pub const fn log_level(&self) -> LogLevel {
        LogLevel::Audit
    }
}

impl PasswordMatchError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}

impl ValidatePasswordError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Empty => LogLevel::Error,
            Self::TooLong => LogLevel::Error,
        }
    }
}
