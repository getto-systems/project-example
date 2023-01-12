use crate::common::api::logger::infra::{LogFilter, LogLevel};

use crate::auth::user::login_id::kernel::data::ValidateLoginIdError;

impl LogFilter for ValidateLoginIdError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::LoginId(err) => err.log_level(),
        }
    }
}
