use crate::{
    auth::{
        ticket::remote::check_nonce::data::ValidateAuthNonceError,
        user::remote::kernel::data::AuthUser,
        login_id::remote::data::ValidateLoginIdError,
        password::{
            remote::kernel::data::{
                PasswordHashError, ValidatePasswordError, VerifyResetTokenEntryError,
            },
            reset::remote::{
                kernel::data::ValidateResetTokenError,
                reset::data::{DecodeResetTokenError, ResetPasswordError},
            },
        },
    },
    z_details::_common::repository::data::RepositoryError,
};

pub enum ResetPasswordEvent {
    Success(AuthUser),
    InvalidReset(ResetPasswordError),
    UserNotFound,
    NonceError(ValidateAuthNonceError),
    RepositoryError(RepositoryError),
    PasswordHashError(PasswordHashError),
    DecodeError(DecodeResetTokenError),
}

const SUCCESS: &'static str = "reset password success";
const ERROR: &'static str = "reset password error";

impl std::fmt::Display for ResetPasswordEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(user) => write!(f, "{}; {}", SUCCESS, user),
            Self::InvalidReset(err) => write!(f, "{}; {}", ERROR, err),
            Self::UserNotFound => write!(f, "{}; user not found", ERROR),
            Self::NonceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
            Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl Into<ResetPasswordEvent> for ValidateLoginIdError {
    fn into(self) -> ResetPasswordEvent {
        ResetPasswordEvent::InvalidReset(ResetPasswordError::InvalidLoginId(self))
    }
}

impl Into<ResetPasswordEvent> for ValidatePasswordError {
    fn into(self) -> ResetPasswordEvent {
        ResetPasswordEvent::InvalidReset(ResetPasswordError::InvalidPassword(self))
    }
}

impl Into<ResetPasswordEvent> for ValidateResetTokenError {
    fn into(self) -> ResetPasswordEvent {
        ResetPasswordEvent::InvalidReset(ResetPasswordError::InvalidResetToken(self))
    }
}

impl Into<ResetPasswordEvent> for VerifyResetTokenEntryError {
    fn into(self) -> ResetPasswordEvent {
        ResetPasswordEvent::InvalidReset(ResetPasswordError::InvalidResetTokenEntry(self))
    }
}
