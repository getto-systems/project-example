use crate::z_details::_common::logger::LogLevel;

use super::super::event::NotifyUnexpectedErrorEvent;

impl NotifyUnexpectedErrorEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Error(_) => LogLevel::Error,
            Self::ValidateError(err) => err.log_level(),
        }
    }
}
