use crate::{
    auth::{
        auth_ticket::_auth::kernel::data::{ExpireDateTime, ValidateAuthNonceError},
        login_id::remote::data::ValidateLoginIdError,
        password::{
            remote::kernel::data::RegisterResetTokenRepositoryError,
            reset::remote::request_token::data::{
                EncodeResetTokenError, NotifyResetTokenError, NotifyResetTokenResponse,
                RequestResetTokenError,
            },
        },
    },
    z_details::_common::repository::data::RepositoryError,
};

pub enum RequestResetTokenEvent {
    TokenExpiresCalculated(ExpireDateTime),
    TokenNotified(NotifyResetTokenResponse),
    Success,
    InvalidRequest(RequestResetTokenError),
    NonceError(ValidateAuthNonceError),
    RepositoryError(RepositoryError),
    EncodeError(EncodeResetTokenError),
    NotifyError(NotifyResetTokenError),
}

const SUCCESS: &'static str = "request reset token success";
const ERROR: &'static str = "request reset token error";

impl std::fmt::Display for RequestResetTokenEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TokenExpiresCalculated(expires) => {
                write!(f, "token expires calculated; {}", expires)
            }
            Self::TokenNotified(response) => {
                write!(f, "token notified; {}", response)
            }
            Self::Success => write!(f, "{}", SUCCESS),
            Self::InvalidRequest(err) => write!(f, "{}; {}", ERROR, err),
            Self::NonceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::EncodeError(err) => write!(f, "{}; {}", ERROR, err),
            Self::NotifyError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

pub fn destination_not_found() -> RequestResetTokenEvent {
    RequestResetTokenEvent::InvalidRequest(RequestResetTokenError::DestinationNotFound)
}

impl Into<RequestResetTokenEvent> for ValidateLoginIdError {
    fn into(self) -> RequestResetTokenEvent {
        RequestResetTokenEvent::InvalidRequest(RequestResetTokenError::InvalidLoginId(self))
    }
}

impl Into<RequestResetTokenEvent> for RegisterResetTokenRepositoryError {
    fn into(self) -> RequestResetTokenEvent {
        match self {
            Self::RepositoryError(err) => RequestResetTokenEvent::RepositoryError(err),
            Self::UserNotFound => {
                RequestResetTokenEvent::InvalidRequest(RequestResetTokenError::UserNotFound)
            }
        }
    }
}
