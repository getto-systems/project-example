use std::fmt::Display;

use super::data::AuthenticatePasswordResponse;
use crate::auth::{
    auth_ticket::_api::kernel::data::ValidateAuthNonceError,
    auth_user::_api::kernel::data::AuthUser,
    login_id::_api::data::ValidateLoginIdError,
    password::_api::kernel::{
        data::{PasswordHashError, ValidatePasswordError},
        infra::VerifyPasswordError,
    },
};
use crate::z_details::_api::{message::data::MessageError, repository::data::RepositoryError};

pub enum AuthenticatePasswordEvent {
    Success(AuthUser),
    UserNotFound,
    InvalidPassword(AuthenticatePasswordResponse),
    NonceError(ValidateAuthNonceError),
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
    MessageError(MessageError),
    ValidateLoginIdError(ValidateLoginIdError),
    ValidatePasswordError(ValidatePasswordError),
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
            Self::ValidateLoginIdError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ValidatePasswordError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<VerifyPasswordError> for AuthenticatePasswordEvent {
    fn from(err: VerifyPasswordError) -> Self {
        match err {
            VerifyPasswordError::PasswordMatchError(err) => Self::PasswordHashError(err),
            VerifyPasswordError::RepositoryError(err) => Self::RepositoryError(err),
        }
    }
}
