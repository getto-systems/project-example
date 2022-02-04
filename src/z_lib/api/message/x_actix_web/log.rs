use crate::z_lib::api::logger::{LogFilter, LogLevel};

use crate::z_lib::api::message::data::MessageError;

impl LogFilter for MessageError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
