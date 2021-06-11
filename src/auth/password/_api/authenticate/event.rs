use std::fmt::Display;

use super::infra::MatchPasswordError;

use super::data::{AuthenticatePasswordError, ConvertPasswordError, PasswordHashError};
use crate::auth::{
    auth_ticket::_api::kernel::data::ValidateAuthNonceError,
    auth_user::_api::kernel::data::AuthUser, login_id::_api::data::ConvertLoginIdError,
};
use crate::z_details::_api::{message::data::MessageError, repository::data::RepositoryError};

pub enum AuthenticatePasswordEvent {
    Success(AuthUser),
    UserNotFound,
    InvalidPassword(AuthenticatePasswordError),
    NonceError(ValidateAuthNonceError),
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
    MessageError(MessageError),
    ConvertLoginIdError(ConvertLoginIdError),
    ConvertPasswordError(ConvertPasswordError),
}

const SUCCESS: &'static str = "authenticate success";
const ERROR: &'static str = "authenticate error";

impl Display for AuthenticatePasswordEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(user) => write!(f, "{}; {}", SUCCESS, user),
            Self::InvalidPassword(_) => write!(f, "{}; password not match", ERROR),
            Self::UserNotFound => write!(f, "{}; user not found", ERROR),
            Self::NonceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ConvertLoginIdError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ConvertPasswordError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<MatchPasswordError> for AuthenticatePasswordEvent {
    fn from(err: MatchPasswordError) -> Self {
        match err {
            MatchPasswordError::PasswordHashError(err) => Self::PasswordHashError(err),
            MatchPasswordError::RepositoryError(err) => Self::RepositoryError(err),
        }
    }
}
