use crate::z_lib::remote::logger::LogLevel;

use super::super::event::DiscardAuthTicketEvent;

impl DiscardAuthTicketEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Audit,
            Self::Validate(event) => event.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
