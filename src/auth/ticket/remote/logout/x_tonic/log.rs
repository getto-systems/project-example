use crate::{
    auth::ticket::remote::logout::action::LogoutEvent,
    z_lib::remote::logger::{LogLevel, LogMessage},
};

use super::super::action::LogoutState;

impl LogMessage for &LogoutState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogoutState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(event) => event.log_level(),
            Self::Logout(event) => event.log_level(),
        }
    }
}

impl LogoutEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Audit,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
