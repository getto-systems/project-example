use super::super::method::IssueAuthTicketEvent;

use crate::z_lib::logger::infra::{LogFilter, LogLevel};

impl LogFilter for IssueAuthTicketEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::ExpansionLimitCalculated(_) => LogLevel::Info,
            Self::Success(_) => LogLevel::Audit,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
