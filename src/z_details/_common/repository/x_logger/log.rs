use crate::z_details::_common::logger::LogLevel;

use crate::z_details::_common::repository::data::RepositoryError;

impl RepositoryError {
    pub const fn log_level(&self) -> LogLevel {
        LogLevel::Error
    }
}
