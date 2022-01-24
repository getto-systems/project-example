use crate::{
    example::outline::remote::get_menu_badge::action::GetOutlineMenuBadgeEvent,
    z_lib::remote::logger::{LogLevel, LogMessage},
};

use super::super::action::GetOutlineMenuBadgeState;

impl LogMessage for &GetOutlineMenuBadgeState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl GetOutlineMenuBadgeState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(err) => err.log_level(),
            Self::GetMenuBadge(event) => event.log_level(),
        }
    }
}

impl GetOutlineMenuBadgeEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Info,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
