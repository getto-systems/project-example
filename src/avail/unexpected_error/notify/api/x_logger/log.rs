use crate::avail::unexpected_error::notify::action::{
    NotifyUnexpectedErrorEvent, NotifyUnexpectedErrorState,
};

use crate::z_lib::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for NotifyUnexpectedErrorState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for NotifyUnexpectedErrorState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::CheckPermission(event) => event.log_level(),
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
