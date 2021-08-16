use crate::z_details::_common::logger::LogLevel;

use crate::z_details::_api::request::data::HeaderError;

impl HeaderError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
