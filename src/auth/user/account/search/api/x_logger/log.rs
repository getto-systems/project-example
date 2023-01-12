use crate::auth::user::account::search::action::{
    SearchAuthUserAccountEvent, SearchAuthUserAccountState,
};

use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for SearchAuthUserAccountState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for SearchAuthUserAccountState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Authorize(event) => event.log_level(),
            Self::Search(event) => event.log_level(),
        }
    }
}

impl LogFilter for SearchAuthUserAccountEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Info,
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
