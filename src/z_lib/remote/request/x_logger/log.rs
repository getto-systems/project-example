use crate::z_lib::remote::logger::LogLevel;

use crate::z_lib::remote::request::data::MetadataError;

impl MetadataError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
