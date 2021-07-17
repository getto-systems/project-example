use crate::z_details::_api::logger::LogLevel;

use crate::z_details::_api::request::data::HeaderError;

impl HeaderError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::NotFound => LogLevel::Debug,
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
