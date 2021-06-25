use crate::x_outside_feature::_api::logger::LogLevel;

use super::super::event::DiscardAuthTicketEvent;

impl DiscardAuthTicketEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Audit,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}