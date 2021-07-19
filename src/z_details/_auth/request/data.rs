use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum MetadataError {
    NotFound,
    Invalid(String),
}

impl Display for MetadataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotFound => write!(f, "metadata not found"),
            Self::Invalid(err) => write!(f, "invalid metadata: {}", err),
        }
    }
}
impl Error for MetadataError {}
