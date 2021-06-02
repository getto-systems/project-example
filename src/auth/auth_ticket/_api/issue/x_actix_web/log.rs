use crate::x_outside_feature::_api::logger::LogLevel;

use super::super::event::IssueAuthTicketEvent;

impl IssueAuthTicketEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Audit,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
