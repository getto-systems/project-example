use crate::z_details::_common::logger::LogLevel;

use super::super::event::IssueAuthTicketEvent;

impl IssueAuthTicketEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::ExpansionLimitCalculated(_) => LogLevel::Info,
            Self::Success(_) => LogLevel::Audit,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
