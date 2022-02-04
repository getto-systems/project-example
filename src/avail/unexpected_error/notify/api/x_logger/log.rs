use crate::z_lib::api::logger::{LogFilter, LogLevel, LogMessage};

use crate::avail::unexpected_error::notify::api::action::{
    NotifyUnexpectedErrorEvent, NotifyUnexpectedErrorState,
};

impl LogMessage for NotifyUnexpectedErrorState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for NotifyUnexpectedErrorState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(event) => event.log_level(),
            Self::Notify(event) => event.log_level(),
        }
    }
}

impl LogFilter for NotifyUnexpectedErrorEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Error(_) => LogLevel::Error,
        }
    }
}
