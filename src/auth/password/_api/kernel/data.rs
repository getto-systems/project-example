use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Clone)]
pub struct ResetToken(String);

impl ResetToken {
    pub fn new(token: String) -> Self {
        Self(token)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn extract(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub enum ValidatePasswordError {
    Empty,
    TooLong,
}

impl Display for ValidatePasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Empty => write!(f, "empty password"),
            Self::TooLong => write!(f, "too long password"),
        }
    }
}
impl Error for ValidatePasswordError {}

#[derive(Debug)]
pub enum PasswordHashError {
    InfraError(String),
}

impl Display for PasswordHashError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "password hash error: {}", err),
        }
    }
}
impl Error for PasswordHashError {}
