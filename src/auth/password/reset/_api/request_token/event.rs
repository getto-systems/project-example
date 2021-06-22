use std::fmt::Display;

use super::data::{
    EncodeResetTokenError, NotifyResetTokenError, NotifyResetTokenResponse,
    RequestResetTokenResponse,
};
use crate::auth::{
    auth_ticket::_api::kernel::data::{ExpireDateTime, ValidateAuthNonceError},
    login_id::_api::data::ValidateLoginIdError,
};
use crate::z_details::_api::{message::data::MessageError, repository::data::RepositoryError};

pub enum RequestResetTokenEvent {
    TokenExpiresCalculated(ExpireDateTime),
    TokenNotified(NotifyResetTokenResponse),
    Success(RequestResetTokenResponse),
    InvalidReset(RequestResetTokenResponse),
    NonceError(ValidateAuthNonceError),
    RepositoryError(RepositoryError),
    MessageError(MessageError),
    EncodeError(EncodeResetTokenError),
    NotifyError(NotifyResetTokenError),
    ValidateLoginIdError(ValidateLoginIdError),
}

const SUCCESS: &'static str = "request reset token success";
const ERROR: &'static str = "request reset token error";

impl Display for RequestResetTokenEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TokenExpiresCalculated(expires) => {
                write!(f, "token expires calculated; {}", expires)
            }
            Self::TokenNotified(response) => {
                write!(f, "token notified; {}", response)
            }
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::InvalidReset(response) => write!(f, "{}; {}", ERROR, response),
            Self::NonceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}; {}", ERROR, err),
            Self::EncodeError(err) => write!(f, "{}; {}", ERROR, err),
            Self::NotifyError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ValidateLoginIdError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}
