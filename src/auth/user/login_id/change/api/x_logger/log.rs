use super::super::action::{OverwriteLoginIdEvent, OverwriteLoginIdState};

use crate::z_lib::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for OverwriteLoginIdState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for OverwriteLoginIdState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Authenticate(event) => event.log_level(),
            Self::Overwrite(event) => event.log_level(),
        }
    }
}

impl LogFilter for OverwriteLoginIdEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Audit,
            Self::Invalid(_) => LogLevel::Error,
            Self::NotFound => LogLevel::Error,
            Self::AlreadyRegistered => LogLevel::Error,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
