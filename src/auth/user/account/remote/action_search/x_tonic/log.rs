use crate::z_lib::remote::logger::{LogLevel, LogMessage};

use super::super::action::SearchUserAccountState;

impl LogMessage for &SearchUserAccountState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl SearchUserAccountState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Search(event) => event.log_level(),
        }
    }
}
