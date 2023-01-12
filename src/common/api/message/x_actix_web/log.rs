use crate::common::api::logger::infra::{LogFilter, LogLevel};

use crate::common::api::message::data::MessageError;

impl LogFilter for MessageError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
