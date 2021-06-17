use crate::x_outside_feature::_api::logger::{LogLevel, LogMessage};

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
            Self::Discard(event) => event.log_level(),
        }
    }
}
