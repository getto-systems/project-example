use crate::z_lib::remote::logger::LogLevel;

use super::super::method::ValidateApiTokenEvent;

impl ValidateApiTokenEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Info,
            Self::MetadataError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
            Self::ServiceError(err) => err.log_level(),
        }
    }
}
