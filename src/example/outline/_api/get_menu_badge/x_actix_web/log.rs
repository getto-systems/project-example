use crate::z_details::_common::logger::LogLevel;

use super::super::event::GetOutlineMenuBadgeEvent;

impl GetOutlineMenuBadgeEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Info,
            Self::MetadataError(err) => err.log_level(),
            Self::ServiceError(err) => err.log_level(),
            Self::MessageError(err) => err.log_level(),
        }
    }
}
