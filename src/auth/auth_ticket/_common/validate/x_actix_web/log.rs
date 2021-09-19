use crate::z_details::_common::logger::LogLevel;

use super::super::data::ValidateApiTokenError;

impl ValidateApiTokenError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::ServiceError(err) => err.log_level(),
            Self::MetadataError(err) => err.log_level(),
        }
    }
}
