use crate::z_lib::remote::logger::{LogLevel, LogMessage};

use super::super::action::SearchAuthUserAccountState;

impl LogMessage for &SearchAuthUserAccountState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl SearchAuthUserAccountState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Search(event) => event.log_level(),
        }
    }
}
