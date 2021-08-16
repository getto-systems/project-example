use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum MetadataError {
    Invalid(String),
}

impl Display for MetadataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Invalid(err) => write!(f, "invalid metadata: {}", err),
        }
    }
}
impl Error for MetadataError {}
