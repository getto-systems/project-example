pub enum ValidateTextError {
    Empty,
    TooLong,
    InvalidEmail,
}

impl std::fmt::Display for ValidateTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Empty => write!(f, "empty"),
            Self::TooLong => write!(f, "too long"),
            Self::InvalidEmail => write!(f, "invalid email"),
        }
    }
}
