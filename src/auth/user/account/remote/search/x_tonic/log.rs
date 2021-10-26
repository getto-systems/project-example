use crate::z_lib::remote::logger::LogLevel;

use super::super::event::SearchAuthUserAccountEvent;

impl SearchAuthUserAccountEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Audit,
            Self::Validate(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}
