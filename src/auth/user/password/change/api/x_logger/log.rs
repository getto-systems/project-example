use super::super::action::{
    ChangePasswordEvent, ChangePasswordState, OverridePasswordEvent, OverridePasswordState,
};

use crate::z_lib::logger::infra::{LogFilter, LogLevel, LogMessage};

use crate::auth::user::password::change::data::{ChangePasswordError, OverridePasswordError};

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

impl LogMessage for OverridePasswordState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for OverridePasswordState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(event) => event.log_level(),
            Self::Override(event) => event.log_level(),
        }
    }
}

impl LogFilter for OverridePasswordEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Audit,
            Self::InvalidPassword(err) => err.log_level(),
            Self::PasswordHashError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}

impl LogFilter for OverridePasswordError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidLoginId(_) => LogLevel::Error,
            Self::InvalidPassword(_) => LogLevel::Error,
            Self::UserNotFound => LogLevel::Error,
        }
    }
}
