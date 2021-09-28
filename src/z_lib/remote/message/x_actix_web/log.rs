use crate::z_lib::remote::logger::LogLevel;

use crate::z_lib::remote::message::data::MessageError;

impl MessageError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
