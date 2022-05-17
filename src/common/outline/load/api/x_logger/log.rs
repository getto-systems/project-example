use crate::common::outline::load::action::{LoadOutlineMenuBadgeEvent, LoadOutlineMenuBadgeState};

use crate::z_lib::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for LoadOutlineMenuBadgeState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for LoadOutlineMenuBadgeState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Authorize(event) => event.log_level(),
            Self::LoadMenuBadge(event) => event.log_level(),
        }
    }
}

impl LogFilter for LoadOutlineMenuBadgeEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Info,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
