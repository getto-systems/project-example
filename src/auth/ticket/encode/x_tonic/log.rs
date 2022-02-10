use crate::auth::ticket::encode::method::EncodeAuthTicketEvent;

use crate::z_lib::logger::infra::{LogFilter, LogLevel};

use crate::auth::ticket::encode::data::EncodeAuthTokenError;

impl LogFilter for EncodeAuthTicketEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::TokenExpiresCalculated(_) => LogLevel::Info,
            Self::Success(_) => LogLevel::Audit,
            Self::TicketNotFound => LogLevel::Audit,
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
