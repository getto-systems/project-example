use crate::z_lib::remote::logger::LogLevel;

use super::super::event::GetOutlineMenuBadgeEvent;

impl GetOutlineMenuBadgeEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Info,
            Self::ValidateError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
