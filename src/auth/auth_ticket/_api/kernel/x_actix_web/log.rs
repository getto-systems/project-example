use crate::{
    auth::auth_ticket::_common::kernel::data::AuthMetadataError,
    z_details::_common::logger::LogLevel,
};

impl AuthMetadataError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::MetadataError(_) => LogLevel::Error,
            Self::DecodeError(err) => err.log_level(),
        }
    }
}
