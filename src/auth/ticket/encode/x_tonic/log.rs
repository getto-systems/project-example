use crate::common::api::logger::infra::{LogFilter, LogLevel};

use crate::auth::ticket::encode::method::EncodeAuthTokenEvent;

use crate::auth::ticket::encode::data::EncodeAuthTokenError;

impl LogFilter for EncodeAuthTokenEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::TokenExpiresCalculated(_) => LogLevel::Info,
            Self::Success(_, _) => LogLevel::Important,
            Self::TicketNotFound => LogLevel::Important,
            Self::RepositoryError(err) => err.log_level(),
            Self::EncodeError(err) => err.log_level(),
        }
    }
}

impl LogFilter for EncodeAuthTokenError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}
