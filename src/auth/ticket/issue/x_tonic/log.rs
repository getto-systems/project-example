use super::super::method::IssueAuthTicketEvent;

use crate::common::api::logger::infra::{LogFilter, LogLevel};

impl LogFilter for IssueAuthTicketEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::ExpansionLimitCalculated(_) => LogLevel::Info,
            Self::Success(_) => LogLevel::Important,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
