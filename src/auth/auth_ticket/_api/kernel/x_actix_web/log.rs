use crate::{
    auth::auth_ticket::_common::kernel::data::AuthServiceMetadataError,
    z_details::_common::logger::LogLevel,
};

impl AuthServiceMetadataError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::MetadataError(_) => LogLevel::Error,
            Self::DecodeError(err) => err.log_level(),
        }
    }
}
