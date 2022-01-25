pub enum ValidatePasswordError {
    Empty,
    TooLong,
}

impl std::fmt::Display for ValidatePasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Empty => write!(f, "empty password"),
            Self::TooLong => write!(f, "too long password"),
        }
    }
}

pub enum PasswordHashError {
    InfraError(String),
}

impl std::fmt::Display for PasswordHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "password hash error: {}", err),
        }
    }
}
