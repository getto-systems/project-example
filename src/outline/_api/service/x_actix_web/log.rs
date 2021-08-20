use crate::z_details::_common::logger::LogLevel;

use crate::outline::_api::service::data::OutlineServiceError;

impl OutlineServiceError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Internal(_) => LogLevel::Error,
            Self::Cancelled(_) => LogLevel::Error,
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}
