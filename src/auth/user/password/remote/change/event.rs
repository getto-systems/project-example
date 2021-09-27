use crate::{
    auth::{
        ticket::remote::check_nonce::data::ValidateAuthNonceError,
        user::password::remote::{
            change::data::ChangePasswordError,
            kernel::data::{
                ChangePasswordRepositoryError, PasswordHashError, ValidatePasswordError,
                VerifyPasswordRepositoryError,
            },
        },
    },
    z_details::_common::repository::data::RepositoryError,
};

use crate::auth::ticket::remote::validate::event::ValidateAuthTokenEvent;

pub enum ChangePasswordEvent {
    Success,
    UserNotFound,
    Validate(ValidateAuthTokenEvent),
    InvalidPassword(ChangePasswordError),
    NonceError(ValidateAuthNonceError),
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "change password success";
const ERROR: &'static str = "change password error";

impl std::fmt::Display for ChangePasswordEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "{}", SUCCESS),
            Self::Validate(event) => event.fmt(f),
            Self::InvalidPassword(response) => write!(f, "{}; {}", ERROR, response),
            Self::UserNotFound => write!(f, "{}; user not found", ERROR),
            Self::NonceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl Into<ChangePasswordEvent> for VerifyPasswordRepositoryError {
    fn into(self) -> ChangePasswordEvent {
        match self {
            Self::PasswordHashError(err) => ChangePasswordEvent::PasswordHashError(err),
            Self::RepositoryError(err) => ChangePasswordEvent::RepositoryError(err),
            Self::PasswordNotFound => {
                ChangePasswordEvent::InvalidPassword(ChangePasswordError::PasswordNotFound)
            }
            Self::PasswordNotMatched => {
                ChangePasswordEvent::InvalidPassword(ChangePasswordError::PasswordNotMatched)
            }
        }
    }
}

impl Into<ChangePasswordEvent> for ChangePasswordRepositoryError {
    fn into(self) -> ChangePasswordEvent {
        match self {
            Self::PasswordHashError(err) => ChangePasswordEvent::PasswordHashError(err),
            Self::RepositoryError(err) => ChangePasswordEvent::RepositoryError(err),
            Self::PasswordNotFound => {
                ChangePasswordEvent::InvalidPassword(ChangePasswordError::PasswordNotFound)
            }
            Self::PasswordNotMatched => {
                ChangePasswordEvent::InvalidPassword(ChangePasswordError::PasswordNotMatched)
            }
        }
    }
}

pub enum ChangePasswordKind {
    Current,
    New,
}

impl Into<ChangePasswordEvent> for (ValidatePasswordError, ChangePasswordKind) {
    fn into(self) -> ChangePasswordEvent {
        match self {
            (err, ChangePasswordKind::Current) => ChangePasswordEvent::InvalidPassword(
                ChangePasswordError::InvalidCurrentPassword(err),
            ),
            (err, ChangePasswordKind::New) => {
                ChangePasswordEvent::InvalidPassword(ChangePasswordError::InvalidNewPassword(err))
            }
        }
    }
}
