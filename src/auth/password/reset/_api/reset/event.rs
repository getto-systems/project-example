use std::fmt::Display;

use crate::auth::{
    auth_ticket::_api::kernel::data::ValidateAuthNonceError,
    auth_user::_api::kernel::data::AuthUser,
    login_id::_api::data::ValidateLoginIdError,
    password::{
        _api::kernel::data::{PasswordHashError, ValidatePasswordError},
        reset::_api::{
            kernel::data::ValidateResetTokenError,
            reset::data::{DecodeResetTokenError, ResetPasswordResponse},
        },
    },
};
use crate::z_details::_api::{message::data::MessageError, repository::data::RepositoryError};

pub enum ResetPasswordEvent {
    Success(AuthUser),
    InvalidReset(ResetPasswordResponse),
    UserNotFound,
    NonceError(ValidateAuthNonceError),
    RepositoryError(RepositoryError),
    PasswordHashError(PasswordHashError),
    MessageError(MessageError),
    DecodeError(DecodeResetTokenError),
    ValidateLoginIdError(ValidateLoginIdError),
    ValidatePasswordError(ValidatePasswordError),
    ValidateResetTokenError(ValidateResetTokenError),
}

const SUCCESS: &'static str = "reset password success";
const ERROR: &'static str = "reset password error";

impl Display for ResetPasswordEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::InvalidReset(_) => write!(f, "{}; invalid reset", ERROR),
            Self::UserNotFound => write!(f, "{}; user not found", ERROR),
            Self::NonceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}; {}", ERROR, err),
            Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ValidateLoginIdError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ValidatePasswordError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ValidateResetTokenError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl Into<ResetPasswordEvent> for Result<ResetPasswordResponse, MessageError> {
    fn into(self) -> ResetPasswordEvent {
        match self {
            Ok(response) => ResetPasswordEvent::InvalidReset(response),
            Err(err) => ResetPasswordEvent::MessageError(err),
        }
    }
}

impl Into<ResetPasswordEvent> for RepositoryError {
    fn into(self) -> ResetPasswordEvent {
        ResetPasswordEvent::RepositoryError(self)
    }
}

impl Into<ResetPasswordEvent> for PasswordHashError {
    fn into(self) -> ResetPasswordEvent {
        ResetPasswordEvent::PasswordHashError(self)
    }
}
