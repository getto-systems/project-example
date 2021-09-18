use crate::z_details::_common::logger::LogLevel;

use super::super::event::ChangePasswordEvent;

impl ChangePasswordEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Result(_) => LogLevel::Audit,
            Self::MetadataError(err) => err.log_level(),
            Self::ServiceError(err) => err.log_level(),
            Self::MessageError(err) => err.log_level(),
        }
    }
}
