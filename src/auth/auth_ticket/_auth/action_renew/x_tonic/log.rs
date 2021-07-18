use crate::z_details::_common::logger::{LogLevel, LogMessage};

use super::super::action::RenewAuthTicketState;

impl LogMessage for &RenewAuthTicketState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl RenewAuthTicketState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(event) => event.log_level(),
            Self::Encode(event) => event.log_level(),
        }
    }
}
