use super::super::action::{ChangeResetTokenDestinationEvent, ChangeResetTokenDestinationState};

use crate::z_lib::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for ChangeResetTokenDestinationState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for ChangeResetTokenDestinationState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(event) => event.log_level(),
            Self::PermissionError(err) => err.log_level(),
            Self::ChangeDestination(event) => event.log_level(),
        }
    }
}

impl LogFilter for ChangeResetTokenDestinationEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Audit,
            Self::Invalid(_) => LogLevel::Error,
            Self::NotFound => LogLevel::Error,
            Self::Conflict => LogLevel::Error,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
