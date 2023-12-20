use crate::{
    auth::user::login_id::kernel::data::ValidateLoginIdError,
    common::api::{repository::data::RepositoryError, notification::data::NotificationError},
};

#[derive(Debug)]
pub struct NotifyResetTokenResponse {
    message_id: String,
}

impl NotifyResetTokenResponse {
    pub fn new(message_id: String) -> Self {
        Self { message_id }
    }
}

impl std::fmt::Display for NotifyResetTokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "message-id: {}", self.message_id)
    }
}

#[derive(Debug)]
pub enum RequestResetPasswordTokenError {
    Invalid(ValidateLoginIdError),
    NotFound,
    RepositoryError(RepositoryError),
    EncodeError(EncodeResetTokenError),
    NotifyError(NotifyResetTokenError),
}

const ERROR: &'static str = "request reset-password-token error";

impl std::fmt::Display for RequestResetPasswordTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
            Self::NotFound => write!(f, "{}; not found", ERROR),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::EncodeError(err) => write!(f, "{}; {}", ERROR, err),
            Self::NotifyError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<ValidateLoginIdError> for RequestResetPasswordTokenError {
    fn from(value: ValidateLoginIdError) -> Self {
        Self::Invalid(value)
    }
}

impl From<RepositoryError> for RequestResetPasswordTokenError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

impl From<EncodeResetTokenError> for RequestResetPasswordTokenError {
    fn from(value: EncodeResetTokenError) -> Self {
        Self::EncodeError(value)
    }
}

impl From<NotifyResetTokenError> for RequestResetPasswordTokenError {
    fn from(value: NotifyResetTokenError) -> Self {
        Self::NotifyError(value)
    }
}

#[derive(Debug)]
pub enum EncodeResetTokenError {
    InfraError(String),
}

impl std::fmt::Display for EncodeResetTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "encode error: {}", err),
        }
    }
}

#[derive(Debug)]
pub enum NotifyResetTokenError {
    NoDestination,
    NotificationError(NotificationError),
}

impl std::fmt::Display for NotifyResetTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NoDestination => write!(f, "no reset token destination"),
            Self::NotificationError(err) => write!(f, "notify reset token error; {}", err),
        }
    }
}

impl From<NotificationError> for NotifyResetTokenError {
    fn from(value: NotificationError) -> Self {
        Self::NotificationError(value)
    }
}
