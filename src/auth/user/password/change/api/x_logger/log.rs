use super::super::action::{
    ChangePasswordEvent, ChangePasswordState, OverwritePasswordEvent, OverwritePasswordState,
};

use crate::z_lib::logger::infra::{LogFilter, LogLevel, LogMessage};

use crate::auth::user::password::change::data::{
    ValidateChangePasswordFieldsError, ValidateOverwritePasswordFieldsError,
};

impl LogMessage for ChangePasswordState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for ChangePasswordState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Authenticate(event) => event.log_level(),
            Self::Change(event) => event.log_level(),
        }
    }
}

impl LogFilter for ChangePasswordEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Audit,
            Self::Invalid(err) => err.log_level(),
            Self::NotFound => LogLevel::Error,
            Self::PasswordNotMatched => LogLevel::Audit,
            Self::PasswordHashError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}

impl LogFilter for ValidateChangePasswordFieldsError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidCurrentPassword(_) => LogLevel::Error,
            Self::InvalidNewPassword(_) => LogLevel::Error,
        }
    }
}

impl LogMessage for OverwritePasswordState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for OverwritePasswordState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Authenticate(event) => event.log_level(),
            Self::Overwrite(event) => event.log_level(),
        }
    }
}

impl LogFilter for OverwritePasswordEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Audit,
            Self::Invalid(err) => err.log_level(),
            Self::NotFound => LogLevel::Error,
            Self::PasswordHashError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}

impl LogFilter for ValidateOverwritePasswordFieldsError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidLoginId(_) => LogLevel::Error,
            Self::InvalidNewPassword(_) => LogLevel::Error,
        }
    }
}
