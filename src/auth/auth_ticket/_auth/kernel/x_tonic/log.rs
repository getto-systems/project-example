use crate::z_details::_common::logger::LogLevel;

use super::super::data::ValidateAuthRolesError;

impl ValidateAuthRolesError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::PermissionDenied(_, _) => LogLevel::Audit,
        }
    }
}
