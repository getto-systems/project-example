use super::super::action::{OverrideLoginIdEvent, OverrideLoginIdState};

use crate::z_lib::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for OverrideLoginIdState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for OverrideLoginIdState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(event) => event.log_level(),
            Self::Override(event) => event.log_level(),
        }
    }
}

impl LogFilter for OverrideLoginIdEvent {
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
