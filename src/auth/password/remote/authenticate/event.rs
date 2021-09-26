use crate::{
    auth::{
        auth_ticket::_auth::kernel::data::ValidateAuthNonceError,
        auth_user::_common::kernel::data::AuthUser,
        login_id::remote::data::ValidateLoginIdError,
        password::remote::{
            authenticate::data::AuthenticatePasswordError,
            kernel::data::{PasswordHashError, ValidatePasswordError, VerifyPasswordRepositoryError},
        },
    },
    z_details::_common::repository::data::RepositoryError,
};

pub enum AuthenticatePasswordEvent {
    Success(AuthUser),
    UserNotFound,
    InvalidPassword(AuthenticatePasswordError),
    NonceError(ValidateAuthNonceError),
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "authenticate password success";
const ERROR: &'static str = "authenticate password error";

impl std::fmt::Display for AuthenticatePasswordEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(user) => write!(f, "{}; {}", SUCCESS, user),
            Self::InvalidPassword(response) => write!(f, "{}; {}", ERROR, response),
            Self::UserNotFound => write!(f, "{}; user not found", ERROR),
            Self::NonceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl Into<AuthenticatePasswordEvent> for VerifyPasswordRepositoryError {
    fn into(self) -> AuthenticatePasswordEvent {
        match self {
            Self::PasswordHashError(err) => AuthenticatePasswordEvent::PasswordHashError(err),
            Self::RepositoryError(err) => AuthenticatePasswordEvent::RepositoryError(err),
            Self::PasswordNotFound => AuthenticatePasswordEvent::InvalidPassword(
                AuthenticatePasswordError::PasswordNotFound,
            ),
            Self::PasswordNotMatched => AuthenticatePasswordEvent::InvalidPassword(
                AuthenticatePasswordError::PasswordNotMatched,
            ),
        }
    }
}

impl Into<AuthenticatePasswordEvent> for ValidateLoginIdError {
    fn into(self) -> AuthenticatePasswordEvent {
        AuthenticatePasswordEvent::InvalidPassword(AuthenticatePasswordError::InvalidLoginId(self))
    }
}

impl Into<AuthenticatePasswordEvent> for ValidatePasswordError {
    fn into(self) -> AuthenticatePasswordEvent {
        AuthenticatePasswordEvent::InvalidPassword(AuthenticatePasswordError::InvalidPassword(self))
    }
}
