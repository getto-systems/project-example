use crate::z_lib::remote::logger::LogLevel;

use crate::auth::ticket::remote::kernel::data::{
    DecodeAuthTokenError, ValidateAuthRolesError,
};

impl DecodeAuthTokenError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Expired => LogLevel::Debug,
            Self::Invalid(_) => LogLevel::Audit,
        }
    }
}

impl ValidateAuthRolesError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::PermissionDenied(_, _) => LogLevel::Audit,
        }
    }
}
