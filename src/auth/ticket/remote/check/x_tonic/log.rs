use crate::z_lib::remote::logger::{LogLevel, LogMessage};

use super::super::action::CheckAuthTicketState;

impl LogMessage for &CheckAuthTicketState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl CheckAuthTicketState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(event) => event.log_level(),
            Self::Encode(event) => event.log_level(),
        }
    }
}
