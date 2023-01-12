use super::super::action::{UnregisterAuthUserAccountEvent, UnregisterAuthUserAccountState};

use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for UnregisterAuthUserAccountState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for UnregisterAuthUserAccountState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Authorize(event) => event.log_level(),
            Self::UnregisterUser(event) => event.log_level(),
        }
    }
}

impl LogFilter for UnregisterAuthUserAccountEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Important,
            Self::Invalid(_) => LogLevel::Error,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
