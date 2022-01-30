use crate::z_lib::remote::logger::{LogLevel, LogFilter};

use crate::example::remote::service::data::ExampleServiceError;

impl LogFilter for ExampleServiceError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Internal(_) => LogLevel::Error,
            Self::Cancelled(_) => LogLevel::Error,
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}
