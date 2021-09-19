use crate::{
    auth::auth_ticket::_common::kernel::data::DecodeAuthTokenError,
    z_details::_common::logger::LogLevel,
};

impl DecodeAuthTokenError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Expired => LogLevel::Debug,
            Self::Invalid(_) => LogLevel::Audit,
        }
    }
}
