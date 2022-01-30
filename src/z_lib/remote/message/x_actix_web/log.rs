use crate::z_lib::remote::logger::{LogFilter, LogLevel};

use crate::z_lib::remote::message::data::MessageError;

impl LogFilter for MessageError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
