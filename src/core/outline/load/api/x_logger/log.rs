use crate::{
    core::outline::load::api::action::LoadOutlineMenuBadgeEvent,
    z_lib::api::logger::{LogFilter, LogLevel, LogMessage},
};

use super::super::action::LoadOutlineMenuBadgeState;

impl LogMessage for LoadOutlineMenuBadgeState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for LoadOutlineMenuBadgeState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(event) => event.log_level(),
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
