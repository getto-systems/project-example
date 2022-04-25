use super::super::action::CheckAuthTicketState;

use crate::z_lib::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for CheckAuthTicketState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for CheckAuthTicketState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Authenticate(event) => event.log_level(),
            Self::Encode(event) => event.log_level(),
        }
    }
}
