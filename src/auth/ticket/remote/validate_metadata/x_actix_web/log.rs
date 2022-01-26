use crate::{
    auth::ticket::remote::validate_metadata::method::ValidateAuthMetadataEvent,
    z_lib::remote::logger::LogLevel,
};

impl ValidateAuthMetadataEvent {
    pub fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Debug,
            Self::MetadataError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
        }
    }
}
