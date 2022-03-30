use super::super::action::{OverrideLoginIdEvent, OverrideLoginIdState};

use crate::z_lib::logger::infra::{LogFilter, LogLevel, LogMessage};

use crate::auth::user::login_id::change::data::OverrideLoginIdError;

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
            Self::Failed(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}

impl LogFilter for OverrideLoginIdError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidLoginId(_) => LogLevel::Error,
            Self::UserNotFound => LogLevel::Error,
            Self::LoginIdAlreadyRegistered => LogLevel::Audit,
        }
    }
}
