use crate::z_details::_common::logger::{LogLevel, LogMessage};

use super::super::action::ChangePasswordState;

impl LogMessage for &ChangePasswordState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl ChangePasswordState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Change(event) => event.log_level(),
        }
    }
}
