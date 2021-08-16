use crate::z_details::_common::logger::{LogLevel, LogMessage};

use super::super::action::AuthenticatePasswordState;

impl LogMessage for &AuthenticatePasswordState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl AuthenticatePasswordState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::MessageError(err) => err.log_level(),
            Self::Authenticate(event) => event.log_level(),
        }
    }
}
