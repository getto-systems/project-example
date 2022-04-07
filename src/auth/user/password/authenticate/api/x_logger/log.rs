use super::super::action::{AuthenticatePasswordEvent, AuthenticatePasswordState};

use crate::{
    auth::user::password::authenticate::data::ValidateAuthenticatePasswordFieldsError,
    z_lib::logger::infra::{LogFilter, LogLevel, LogMessage},
};

impl LogMessage for AuthenticatePasswordState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for AuthenticatePasswordState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Authenticate(event) => event.log_level(),
            Self::ValidateNonce(event) => event.log_level(),
            Self::Issue(event) => event.log_level(),
            Self::Encode(event) => event.log_level(),
        }
    }
}

impl LogFilter for AuthenticatePasswordEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Audit,
            Self::Invalid(err) => err.log_level(),
            Self::NotFound => LogLevel::Error,
            Self::PasswordNotMatched => LogLevel::Audit,
            Self::PasswordHashError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}

impl LogFilter for ValidateAuthenticatePasswordFieldsError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidLoginId(err) => err.log_level(),
            Self::InvalidPassword(err) => err.log_level(),
        }
    }
}
