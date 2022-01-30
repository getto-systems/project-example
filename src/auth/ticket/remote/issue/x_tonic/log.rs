use crate::z_lib::remote::logger::{LogLevel, LogFilter};

use super::super::method::IssueAuthTicketEvent;

impl LogFilter for IssueAuthTicketEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::ExpansionLimitCalculated(_) => LogLevel::Info,
            Self::Success(_) => LogLevel::Audit,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
