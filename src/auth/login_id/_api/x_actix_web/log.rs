use crate::x_outside_feature::_api::logger::LogLevel;

use super::super::data::ConvertLoginIdError;

impl ConvertLoginIdError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Empty => LogLevel::Error,
            Self::TooLong => LogLevel::Error,
        }
    }
}
