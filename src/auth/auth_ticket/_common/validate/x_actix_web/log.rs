use crate::z_details::_common::logger::LogLevel;

use super::super::data::ValidateApiTokenError;

impl ValidateApiTokenError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::MetadataError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
            Self::ServiceError(err) => err.log_level(),
        }
    }
}
