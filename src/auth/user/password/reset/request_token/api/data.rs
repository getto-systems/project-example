use crate::{
    auth::user::login_id::kernel::data::ValidateLoginIdError,
    z_lib::api::repository::data::RepositoryError,
};

pub enum RequestResetTokenError {
    InvalidLoginId(ValidateLoginIdError),
    DestinationNotFound,
    UserNotFound,
}

impl std::fmt::Display for RequestResetTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => write!(f, "invalid login id: {}", err),
            Self::DestinationNotFound => write!(f, "destination not found"),
            Self::UserNotFound => write!(f, "user not found"),
        }
    }
}

pub enum RegisterResetTokenRepositoryError {
    RepositoryError(RepositoryError),
    UserNotFound,
}

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

pub enum NotifyResetTokenError {
    InfraError(String),
}

impl std::fmt::Display for NotifyResetTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "notify reset token error; {}", err),
        }
    }
}
