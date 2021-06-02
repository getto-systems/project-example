use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum HeaderError {
    NotFound,
    Invalid(String),
}

impl Display for HeaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotFound => write!(f, "header not found"),
            Self::Invalid(err) => write!(f, "invalid header: {}", err),
        }
    }
}
impl Error for HeaderError {}
