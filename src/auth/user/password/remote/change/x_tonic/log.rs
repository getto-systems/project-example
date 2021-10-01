use crate::z_lib::remote::logger::LogLevel;

use crate::auth::user::password::remote::change::data::ChangePasswordError;

use super::super::event::ChangePasswordEvent;

impl ChangePasswordEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Audit,
            Self::UserNotFound => LogLevel::Error,
            Self::Validate(event) => event.log_level(),
            Self::InvalidPassword(err) => err.log_level(),
            Self::NonceError(err) => err.log_level(),
            Self::PasswordHashError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}

impl ChangePasswordError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidCurrentPassword(_) => LogLevel::Error,
            Self::InvalidNewPassword(_) => LogLevel::Error,
            Self::PasswordNotFound => LogLevel::Error,
            Self::PasswordNotMatched => LogLevel::Audit,
        }
    }
}