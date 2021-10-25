use crate::z_lib::remote::logger::LogLevel;

use super::super::event::SearchUserAccountEvent;

impl SearchUserAccountEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Audit,
            Self::Validate(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
