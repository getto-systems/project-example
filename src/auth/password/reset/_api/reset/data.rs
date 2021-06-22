use std::{
    error::Error,
    fmt::{Display, Formatter},
};

pub enum ResetPasswordResponse {
    NotFound(String),
    AlreadyReset(String),
    Expired(String),
    InvalidLoginId(String),
}

impl Display for ResetPasswordResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotFound(_) => write!(f, "reset token not found"),
            Self::AlreadyReset(_) => write!(f, "already reset"),
            Self::Expired(_) => write!(f, "reset token expired"),
            Self::InvalidLoginId(_) => write!(f, "invalid login id"),
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
