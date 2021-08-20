use crate::z_details::_common::logger::LogLevel;

use super::super::event::GetOutlineMenuBadgeEvent;

impl GetOutlineMenuBadgeEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Authorized(_) => LogLevel::Info,
            Self::Success(_) => LogLevel::Info,
            Self::ValidateApiTokenError(err) => err.log_level(),
            Self::ServiceError(err) => err.log_level(),
            Self::HeaderError(err) => err.log_level(),
            Self::MessageError(err) => err.log_level(),
        }
    }
}
