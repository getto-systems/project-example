pub enum MessageError {
    Invalid(String),
}

impl std::fmt::Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Invalid(err) => write!(f, "invalid message: {}", err),
        }
    }
}
