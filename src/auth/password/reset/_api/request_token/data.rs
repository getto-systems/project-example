use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::auth::auth_user::_api::kernel::data::AuthUserId;

pub struct RequestResetTokenResponse {
    pub message: String,
}

#[derive(Clone)]
pub struct ResetTokenDestination {
    user_id: AuthUserId,
    email: String,
}

impl ResetTokenDestination {
    pub fn extract(self) -> ResetTokenDestinationExtract {
        ResetTokenDestinationExtract {
            user_id: self.user_id.extract(),
            email: self.email,
        }
    }

    pub fn as_user_id(&self) -> &AuthUserId {
        &self.user_id
    }

    pub fn into_email(self) -> String {
        self.email
    }
}

pub struct ResetTokenDestinationExtract {
    pub user_id: String,
    pub email: String,
}

impl Into<ResetTokenDestination> for ResetTokenDestinationExtract {
    fn into(self) -> ResetTokenDestination {
        ResetTokenDestination {
            user_id: AuthUserId::new(self.user_id),
            email: self.email,
        }
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
