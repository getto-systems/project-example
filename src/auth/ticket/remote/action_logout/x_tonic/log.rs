use crate::z_details::_common::logger::{LogLevel, LogMessage};

use super::super::action::LogoutState;

impl LogMessage for &LogoutState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogoutState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Discard(event) => event.log_level(),
        }
    }
}
