use crate::{
    auth::{
        _common::service::data::AuthServiceError,
        auth_ticket::_common::kernel::data::AuthServiceMetadataError,
        password::reset::_api::reset::data::ResetPasswordMessage,
    },
    z_details::_api::message::data::MessageError,
};

pub enum ResetPasswordEvent {
    Result(ResetPasswordMessage),
    MetadataError(AuthServiceMetadataError),
    ServiceError(AuthServiceError),
    MessageError(MessageError),
}

const SUCCESS: &'static str = "reset password";
const ERROR: &'static str = "reset password error";

impl std::fmt::Display for ResetPasswordEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Result(message) => write!(f, "{}", message),
            Self::MetadataError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl std::fmt::Display for ResetPasswordMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::InvalidReset(_) => write!(f, "{}; invalid reset", ERROR),
            Self::AlreadyReset(_) => write!(f, "{}; already reset", ERROR),
        }
    }
}
