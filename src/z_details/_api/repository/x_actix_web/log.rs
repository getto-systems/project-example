use crate::z_details::_api::logger::LogLevel;

use crate::z_details::_api::repository::data::RepositoryError;

impl RepositoryError {
    pub const fn log_level(&self) -> LogLevel {
        LogLevel::Error
    }
}
