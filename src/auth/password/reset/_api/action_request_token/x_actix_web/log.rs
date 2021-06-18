use crate::x_outside_feature::_api::logger::{LogLevel, LogMessage};

use super::super::action::RequestResetTokenState;

impl LogMessage for &RequestResetTokenState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl RequestResetTokenState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::RequestToken(event) => event.log_level(),
        }
    }
}
