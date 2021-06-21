use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use super::convert::validate_reset_token;

pub struct ResetTokenEncoded(String);

impl ResetTokenEncoded {
    pub fn validate(token: String) -> Result<Self, ValidateResetTokenError> {
        validate_reset_token(&token)?;
        Ok(Self(token))
    }

    pub fn new(token: String) -> Self {
        Self(token)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug)]
pub enum ValidateResetTokenError {
    Empty,
}

impl Display for ValidateResetTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Empty => write!(f, "empty password reset token"),
        }
    }
}
impl Error for ValidateResetTokenError {}
