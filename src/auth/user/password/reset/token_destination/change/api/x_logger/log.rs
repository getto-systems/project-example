use super::super::action::{ChangeResetTokenDestinationEvent, ChangeResetTokenDestinationState};

use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

use crate::auth::user::password::reset::token_destination::change::data::ValidateChangeResetTokenDestinationFieldsError;

impl LogMessage for ChangeResetTokenDestinationState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for ChangeResetTokenDestinationState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Authorize(event) => event.log_level(),
            Self::ChangeDestination(event) => event.log_level(),
        }
    }
}

impl LogFilter for ChangeResetTokenDestinationEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Important,
            Self::Invalid(err) => err.log_level(),
            Self::NotFound => LogLevel::Error,
            Self::Conflict => LogLevel::Error,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}

impl LogFilter for ValidateChangeResetTokenDestinationFieldsError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidLoginId(err) => err.log_level(),
            Self::InvalidFrom(err) => err.log_level(),
            Self::InvalidTo(err) => err.log_level(),
        }
    }
}
