use crate::z_details::_common::logger::LogLevel;

use crate::z_details::_auth::request::data::MetadataError;

impl MetadataError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::NotFound => LogLevel::Error,
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
