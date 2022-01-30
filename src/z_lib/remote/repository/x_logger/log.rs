use crate::z_lib::remote::logger::{LogFilter, LogLevel};

use crate::z_lib::remote::repository::data::RepositoryError;

impl LogFilter for RepositoryError {
    fn log_level(&self) -> LogLevel {
        LogLevel::Error
    }
}
