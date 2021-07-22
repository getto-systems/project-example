use std::fmt::Display;

use crate::{
    auth::{
        _api::service::data::ServiceError,
        password::_api::authenticate::data::AuthenticatePasswordMessage,
    },
    z_details::_api::{message::data::MessageError, request::data::HeaderError},
};

pub enum AuthenticatePasswordEvent {
    Result(AuthenticatePasswordMessage),
    HeaderError(HeaderError),
    ServiceError(ServiceError),
    MessageError(MessageError),
}

const SUCCESS: &'static str = "authenticate password success";
const ERROR: &'static str = "authenticate password error";

impl Display for AuthenticatePasswordEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Result(message) => write!(f, "{}", message),
            Self::HeaderError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl Display for AuthenticatePasswordMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::InvalidPassword(_) => write!(f, "{}; invalid password", ERROR),
        }
    }
}
