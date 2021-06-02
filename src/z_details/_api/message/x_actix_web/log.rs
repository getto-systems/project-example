use crate::x_outside_feature::_api::logger::LogLevel;

use super::super::data::MessageError;

impl MessageError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
