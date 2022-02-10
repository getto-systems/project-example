use crate::z_lib::logger::infra::{LogFilter, LogLevel};

use crate::z_lib::message::data::MessageError;

impl LogFilter for MessageError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
