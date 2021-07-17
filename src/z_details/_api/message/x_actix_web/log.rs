use crate::z_details::_api::logger::LogLevel;

use crate::z_details::_api::message::data::MessageError;

impl MessageError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
