use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum MessageError {
    Invalid(String),
}

impl Display for MessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Invalid(err) => write!(f, "invalid message: {}", err),
        }
    }
}
impl Error for MessageError {}
