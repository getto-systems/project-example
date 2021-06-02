use crate::{auth::password::_api::authenticate::data::{AuthenticatePasswordError, ConvertPasswordError, PasswordHashError}, x_outside_feature::_api::logger::LogLevel};

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
            Self::ConvertLoginIdError(err) => err.log_level(),
            Self::ConvertPasswordError(err) => err.log_level(),
        }
    }
}

impl AuthenticatePasswordError {
    pub const fn log_level(&self) -> LogLevel {
        LogLevel::Audit
    }
}

impl PasswordHashError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}

impl ConvertPasswordError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Empty => LogLevel::Error,
            Self::TooLong => LogLevel::Error,
        }
    }
}
