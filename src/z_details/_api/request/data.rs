pub enum HeaderError {
    Invalid(String),
}

impl std::fmt::Display for HeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Invalid(err) => write!(f, "invalid header: {}", err),
        }
    }
}
