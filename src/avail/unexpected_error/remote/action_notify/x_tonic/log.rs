use crate::z_lib::remote::logger::{LogLevel, LogMessage};

use super::super::action::NotifyUnexpectedErrorState;

impl LogMessage for &NotifyUnexpectedErrorState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl NotifyUnexpectedErrorState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Notify(event) => event.log_level(),
        }
    }
}
