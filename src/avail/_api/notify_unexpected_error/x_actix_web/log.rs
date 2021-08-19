use crate::z_details::_common::logger::LogLevel;

use super::super::event::NotifyUnexpectedErrorEvent;

impl NotifyUnexpectedErrorEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Authorized(_) => LogLevel::Info,
            Self::Notice(_) => LogLevel::Error,
            Self::ValidateApiTokenError(err) => err.log_level(),
            Self::ServiceError(err) => err.log_level(),
            Self::HeaderError(err) => err.log_level(),
        }
    }
}
