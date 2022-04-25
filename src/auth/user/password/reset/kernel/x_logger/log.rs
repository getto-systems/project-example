use crate::z_lib::logger::infra::{LogFilter, LogLevel};

use crate::auth::user::password::reset::kernel::data::{
    ValidateResetTokenDestinationEmailError, ValidateResetTokenDestinationError,
};

impl LogFilter for ValidateResetTokenDestinationError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Email(err) => err.log_level(),
        }
    }
}

impl LogFilter for ValidateResetTokenDestinationEmailError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Text(err) => err.log_level(),
        }
    }
}
