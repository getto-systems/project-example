use super::super::action::{ModifyAuthUserAccountEvent, ModifyAuthUserAccountState};

use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for ModifyAuthUserAccountState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for ModifyAuthUserAccountState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Authorize(event) => event.log_level(),
            Self::ModifyUser(event) => event.log_level(),
        }
    }
}

impl LogFilter for ModifyAuthUserAccountEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Important,
            Self::Invalid(_) => LogLevel::Error,
            Self::NotFound => LogLevel::Error,
            Self::Conflict => LogLevel::Error,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
