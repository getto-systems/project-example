use crate::{
    auth::ticket::remote::logout::action::LogoutEvent,
    z_lib::remote::logger::{LogLevel, LogMessage, LogFilter},
};

use super::super::action::LogoutState;

impl LogMessage for LogoutState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for LogoutState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(event) => event.log_level(),
            Self::Logout(event) => event.log_level(),
        }
    }
}

impl LogFilter for LogoutEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Audit,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
