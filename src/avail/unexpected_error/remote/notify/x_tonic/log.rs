use crate::{
    avail::unexpected_error::remote::notify::action::NotifyUnexpectedErrorEvent,
    z_lib::remote::logger::{LogLevel, LogMessage},
};

use super::super::action::NotifyUnexpectedErrorState;

impl LogMessage for &NotifyUnexpectedErrorState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl NotifyUnexpectedErrorState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(event) => event.log_level(),
            Self::Notify(event) => event.log_level(),
        }
    }
}

impl NotifyUnexpectedErrorEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Error(_) => LogLevel::Error,
        }
    }
}
