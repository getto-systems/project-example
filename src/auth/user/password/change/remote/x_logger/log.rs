use crate::z_lib::remote::logger::{LogFilter, LogLevel, LogMessage};

use super::super::action::{ChangePasswordEvent, ChangePasswordState};

use crate::auth::user::password::change::remote::data::ChangePasswordError;

impl LogMessage for ChangePasswordState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for ChangePasswordState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(event) => event.log_level(),
            Self::Change(event) => event.log_level(),
        }
    }
}

impl LogFilter for ChangePasswordEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Audit,
            Self::UserNotFound => LogLevel::Error,
            Self::InvalidPassword(err) => err.log_level(),
            Self::PasswordHashError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}

impl LogFilter for ChangePasswordError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidCurrentPassword(_) => LogLevel::Error,
            Self::InvalidNewPassword(_) => LogLevel::Error,
            Self::PasswordNotFound => LogLevel::Error,
            Self::PasswordNotMatched => LogLevel::Audit,
        }
    }
}
