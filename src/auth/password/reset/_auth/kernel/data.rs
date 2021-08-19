pub struct ResetTokenEncoded(String);

impl ResetTokenEncoded {
    pub fn validate(token: impl ResetTokenEncodedExtract) -> Result<Self, ValidateResetTokenError> {
        Ok(Self(token.validate()?))
    }

    pub const fn new(token: String) -> Self {
        Self(token)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub trait ResetTokenEncodedExtract {
    fn validate(self) -> Result<String, ValidateResetTokenError>;
}

pub enum ValidateResetTokenError {
    Empty,
}

impl std::fmt::Display for ValidateResetTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Empty => write!(f, "empty reset token"),
        }
    }
}
