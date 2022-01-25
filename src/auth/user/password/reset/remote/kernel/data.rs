#[derive(Clone)]
pub struct ResetToken(String);

impl ResetToken {
    pub const fn new(token: String) -> Self {
        Self(token)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn extract(self) -> String {
        self.0
    }
}

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

#[derive(Clone)]
pub struct ResetTokenDestination {
    email: String,
}

impl ResetTokenDestination {
    #[cfg(test)]
    pub fn extract(self) -> ResetTokenDestinationExtract {
        ResetTokenDestinationExtract { email: self.email }
    }

    pub fn into_email(self) -> String {
        self.email
    }
}

#[derive(Clone)]
pub struct ResetTokenDestinationExtract {
    pub email: String,
}

impl ResetTokenDestinationExtract {
    pub(in crate::auth) fn restore(self) -> ResetTokenDestination {
        ResetTokenDestination { email: self.email }
    }
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
