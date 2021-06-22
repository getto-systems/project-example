use std::{
    error::Error,
    fmt::{Display, Formatter},
};

pub struct ResetPasswordResponse {
    pub message: String,
}

#[derive(Debug)]
pub enum DecodeResetTokenError {
    Expired,
    Invalid(String),
}

impl Display for DecodeResetTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Expired => write!(f, "token expired"),
            Self::Invalid(err) => write!(f, "decode error; {}", err),
        }
    }
}
impl Error for DecodeResetTokenError {}
