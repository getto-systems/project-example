use crate::x_outside_feature::_api::logger::LogLevel;

use super::super::data::HeaderError;

impl HeaderError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::NotFound => LogLevel::Debug,
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
