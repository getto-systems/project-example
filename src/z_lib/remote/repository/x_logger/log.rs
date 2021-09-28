use crate::z_lib::remote::logger::LogLevel;

use crate::z_lib::remote::repository::data::RepositoryError;

impl RepositoryError {
    pub const fn log_level(&self) -> LogLevel {
        LogLevel::Error
    }
}
