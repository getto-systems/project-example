use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

use crate::auth::ticket::logout::action::LogoutEvent;

use super::super::action::LogoutState;

impl LogMessage for LogoutState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for LogoutState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::AuthenticateWithToken(event) => event.log_level(),
            Self::Logout(event) => event.log_level(),
        }
    }
}

impl LogFilter for LogoutEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Important,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
