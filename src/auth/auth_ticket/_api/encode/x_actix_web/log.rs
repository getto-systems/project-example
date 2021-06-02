use crate::x_outside_feature::_api::logger::LogLevel;

use super::super::event::EncodeAuthTicketEvent;

use super::super::data::EncodeAuthTokenError;

impl EncodeAuthTicketEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Audit,
            Self::TicketNotFound => LogLevel::Audit,
            Self::RepositoryError(err) => err.log_level(),
            Self::EncodeError(err) => err.log_level(),
            Self::MessageError(err) => err.log_level(),
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
