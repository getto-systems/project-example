use crate::z_lib::remote::logger::{LogLevel, LogMessage, LogFilter};

use super::super::action::CheckAuthTicketState;

impl LogMessage for CheckAuthTicketState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for CheckAuthTicketState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(event) => event.log_level(),
            Self::Encode(event) => event.log_level(),
        }
    }
}
