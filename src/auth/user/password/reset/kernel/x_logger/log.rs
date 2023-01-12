use crate::common::api::logger::infra::{LogFilter, LogLevel};

use crate::auth::user::password::reset::kernel::data::ValidateResetPasswordTokenDestinationError;

impl LogFilter for ValidateResetPasswordTokenDestinationError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::NotFound => LogLevel::Error,
            Self::Email(err) => err.log_level(),
        }
    }
}
