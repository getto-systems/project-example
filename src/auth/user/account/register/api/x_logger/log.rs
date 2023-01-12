use super::super::action::{RegisterAuthUserAccountEvent, RegisterAuthUserAccountState};

use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for RegisterAuthUserAccountState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for RegisterAuthUserAccountState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Authorize(event) => event.log_level(),
            Self::RegisterUser(event) => event.log_level(),
        }
    }
}

impl LogFilter for RegisterAuthUserAccountEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Important,
            Self::Invalid(_) => LogLevel::Error,
            Self::LoginIdAlreadyRegistered => LogLevel::Error,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
