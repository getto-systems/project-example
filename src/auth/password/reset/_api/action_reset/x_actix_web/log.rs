use crate::z_details::_common::logger::{LogLevel, LogMessage};

use super::super::action::ResetPasswordState;

impl LogMessage for &ResetPasswordState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl ResetPasswordState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Reset(event) => event.log_level(),
            Self::MessageError(err) => err.log_level(),
        }
    }
}
