use crate::z_lib::remote::logger::{LogLevel, LogMessage};

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
            Self::Issue(event) => event.log_level(),
            Self::Encode(event) => event.log_level(),
        }
    }
}
