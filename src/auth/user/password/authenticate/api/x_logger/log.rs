use crate::z_lib::logger::{LogFilter, LogLevel, LogMessage};

use super::super::action::{AuthenticatePasswordEvent, AuthenticatePasswordState};

use super::super::data::AuthenticatePasswordError;

impl LogMessage for AuthenticatePasswordState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for AuthenticatePasswordState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Authenticate(event) => event.log_level(),
            Self::ValidateNonce(event) => event.log_level(),
            Self::Issue(event) => event.log_level(),
            Self::Encode(event) => event.log_level(),
        }
    }
}

impl LogFilter for AuthenticatePasswordEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Audit,
            Self::UserNotFound => LogLevel::Error,
            Self::InvalidPassword(err) => err.log_level(),
            Self::PasswordHashError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}

impl LogFilter for AuthenticatePasswordError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidLoginId(_) => LogLevel::Error,
            Self::InvalidPassword(_) => LogLevel::Error,
            Self::PasswordNotFound => LogLevel::Error,
            Self::PasswordNotMatched => LogLevel::Audit,
        }
    }
}
