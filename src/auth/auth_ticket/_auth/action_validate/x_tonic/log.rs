use crate::z_details::_common::logger::{LogLevel, LogMessage};

use super::super::action::ValidateApiTokenState;

impl LogMessage for &ValidateApiTokenState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl ValidateApiTokenState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(event) => event.log_level(),
            Self::Success(_) => LogLevel::Info,
        }
    }
}
