use crate::z_details::_common::logger::LogLevel;

use crate::auth::auth_ticket::_auth::encode::event::EncodeAuthTicketEvent;

use crate::auth::auth_ticket::_auth::encode::data::EncodeAuthTokenError;

impl EncodeAuthTicketEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::TokenExpiresCalculated(_) => LogLevel::Info,
            Self::Success(_) => LogLevel::Audit,
            Self::TicketNotFound => LogLevel::Audit,
            Self::RepositoryError(err) => err.log_level(),
            Self::EncodeError(err) => err.log_level(),
        }
    }
}

impl EncodeAuthTokenError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}
