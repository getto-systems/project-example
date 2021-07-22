use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::auth::{
    login_id::_auth::data::ValidateLoginIdError,
    password::{
        _auth::kernel::data::{ValidatePasswordError, VerifyResetTokenEntryError},
        reset::_auth::kernel::data::ValidateResetTokenError,
    },
};

pub enum ResetPasswordError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidPassword(ValidatePasswordError),
    InvalidResetToken(ValidateResetTokenError),
    InvalidResetTokenEntry(VerifyResetTokenEntryError),
}

impl Display for ResetPasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => write!(f, "invalid login id: {}", err),
            Self::InvalidPassword(err) => write!(f, "invalid password: {}", err),
            Self::InvalidResetToken(err) => write!(f, "invalid reset token: {}", err),
            Self::InvalidResetTokenEntry(err) => write!(f, "invalid reset token entry: {}", err),
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
