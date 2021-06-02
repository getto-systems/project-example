use crate::x_outside_feature::_api::logger::LogLevel;

use super::super::data::RepositoryError;

impl RepositoryError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}
