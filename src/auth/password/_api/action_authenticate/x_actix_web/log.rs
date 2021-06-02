use crate::x_outside_feature::_api::logger::{LogLevel, LogMessage};

use super::super::action::AuthenticatePasswordState;

impl LogMessage for &AuthenticatePasswordState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl AuthenticatePasswordState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Authenticate(event) => event.log_level(),
            Self::Issue(event) => event.log_level(),
            Self::Encode(event) => event.log_level(),
        }
    }
}
