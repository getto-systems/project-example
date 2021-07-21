use crate::z_details::_common::logger::LogLevel;

use super::super::event::AuthenticatePasswordEvent;

impl AuthenticatePasswordEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Audit,
            Self::HeaderError(err) => err.log_level(),
            Self::ServiceError(err) => err.log_level(),
            Self::MessageError(err) => err.log_level(),
        }
    }
}
