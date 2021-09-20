use crate::{
    auth::{
        _common::service::data::AuthServiceError,
        auth_ticket::_common::kernel::data::AuthMetadataError,
        password::reset::_api::request_token::data::RequestResetTokenResult,
    },
    z_details::_api::message::data::MessageError,
};

pub enum RequestResetTokenEvent {
    Result(RequestResetTokenResult),
    MetadataError(AuthMetadataError),
    ServiceError(AuthServiceError),
    MessageError(MessageError),
}

const SUCCESS: &'static str = "request reset token";
const ERROR: &'static str = "request reset token error";

impl std::fmt::Display for RequestResetTokenEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Result(message) => write!(f, "{}", message),
            Self::MetadataError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl std::fmt::Display for RequestResetTokenResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::InvalidRequest(_) => write!(f, "{}; invalid request", ERROR),
        }
    }
}
