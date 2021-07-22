use crate::z_details::_common::logger::LogLevel;

use crate::auth::password::reset::_api::request_token::event::RequestResetTokenEvent;

impl RequestResetTokenEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Result(_) => LogLevel::Audit,
            Self::HeaderError(err) => err.log_level(),
            Self::ServiceError(err) => err.log_level(),
            Self::MessageError(err) => err.log_level(),
        }
    }
}
