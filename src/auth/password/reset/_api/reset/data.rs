use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum DecodeResetTokenError {
    InfraError(String),
}

impl Display for DecodeResetTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "decode error: {}", err),
        }
    }
}
impl Error for DecodeResetTokenError {}
