use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::auth::login_id::_auth::data::ValidateLoginIdError;

pub enum RequestResetTokenError {
    InvalidLoginId(ValidateLoginIdError),
    DestinationNotFound,
    UserNotFound,
}

impl Display for RequestResetTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => write!(f, "invalid login id: {}", err),
            Self::DestinationNotFound => write!(f, "destination not found"),
            Self::UserNotFound => write!(f, "user not found"),
        }
    }
}

#[derive(Clone)]
pub struct ResetTokenDestination {
    email: String,
}

impl ResetTokenDestination {
    pub fn into_email(self) -> String {
        self.email
    }
}

pub struct ResetTokenDestinationExtract {
    pub email: String,
}

impl Into<ResetTokenDestination> for ResetTokenDestinationExtract {
    fn into(self) -> ResetTokenDestination {
        ResetTokenDestination { email: self.email }
    }
}

pub struct NotifyResetTokenResponse {
    message_id: String,
}

impl NotifyResetTokenResponse {
    pub fn new(message_id: String) -> Self {
        Self { message_id }
    }
}

impl Display for NotifyResetTokenResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "message-id: {}", self.message_id)
    }
}

#[derive(Debug)]
pub enum EncodeResetTokenError {
    InfraError(String),
}

impl Display for EncodeResetTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "encode error: {}", err),
        }
    }
}
impl Error for EncodeResetTokenError {}

#[derive(Debug)]
pub enum NotifyResetTokenError {
    InfraError(String),
}

impl Display for NotifyResetTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "notify reset token error; {}", err),
        }
    }
}
impl Error for NotifyResetTokenError {}
