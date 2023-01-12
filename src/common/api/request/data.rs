pub enum MetadataError {
    Invalid(String),
}

impl std::fmt::Display for MetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Invalid(err) => write!(f, "invalid metadata: {}", err),
        }
    }
}