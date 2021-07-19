use crate::z_details::_common::logger::LogLevel;

use super::super::event::LogoutEvent;

impl LogoutEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Audit,
            Self::ServiceError(err) => err.log_level(),
            Self::HeaderError(err) => err.log_level(),
        }
    }
}
