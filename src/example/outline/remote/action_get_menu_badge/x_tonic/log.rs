use crate::z_lib::remote::logger::{LogLevel, LogMessage};

use super::super::action::GetOutlineMenuBadgeState;

impl LogMessage for &GetOutlineMenuBadgeState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl GetOutlineMenuBadgeState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::GetMenuBadge(event) => event.log_level(),
        }
    }
}
