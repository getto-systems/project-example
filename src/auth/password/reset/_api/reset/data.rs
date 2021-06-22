use std::{
    error::Error,
    fmt::{Display, Formatter},
};

pub struct ResetPasswordResponse {
    kind: ResponseKind,
    pub message: String,
}

impl ResetPasswordResponse {
    pub fn not_found(message: String) -> Self {
        Self {
            kind: ResponseKind::NotFound,
            message,
        }
    }
    pub fn already_reset(message: String) -> Self {
        Self {
            kind: ResponseKind::AlreadyReset,
            message,
        }
    }
    pub fn expired(message: String) -> Self {
        Self {
            kind: ResponseKind::Expired,
            message,
        }
    }
    pub fn invalid_login_id(message: String) -> Self {
        Self {
            kind: ResponseKind::InvalidLoginId,
            message,
        }
    }
}

enum ResponseKind {
    NotFound,
    AlreadyReset,
    Expired,
    InvalidLoginId,
}

impl Display for ResetPasswordResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.kind {
            ResponseKind::NotFound => write!(f, "reset token not found"),
            ResponseKind::AlreadyReset => write!(f, "already reset"),
            ResponseKind::Expired => write!(f, "reset token expired"),
            ResponseKind::InvalidLoginId => write!(f, "invalid login id"),
        }
    }
}

#[derive(Debug)]
pub enum DecodeResetTokenError {
    Expired,
    Invalid(String),
}

impl Display for DecodeResetTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Expired => write!(f, "reset token expired"),
            Self::Invalid(err) => write!(f, "decode error; {}", err),
        }
    }
}
impl Error for DecodeResetTokenError {}
