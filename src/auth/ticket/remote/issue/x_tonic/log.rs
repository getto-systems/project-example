use crate::z_lib::remote::logger::LogLevel;

use super::super::method::IssueAuthTicketEvent;

impl IssueAuthTicketEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::ExpansionLimitCalculated(_) => LogLevel::Info,
            Self::Success(_) => LogLevel::Audit,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
