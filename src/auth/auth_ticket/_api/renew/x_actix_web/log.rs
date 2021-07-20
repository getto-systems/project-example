use crate::z_details::_common::logger::LogLevel;

use super::super::event::RenewAuthTicketEvent;

impl RenewAuthTicketEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Audit,
            Self::ServiceError(err) => err.log_level(),
            Self::HeaderError(err) => err.log_level(),
            Self::MessageError(err) => err.log_level(),
        }
    }
}
