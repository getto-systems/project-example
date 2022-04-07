#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ResetToken(String);

impl ResetToken {
    pub const fn restore(token: String) -> Self {
        Self(token)
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

#[derive(Clone, PartialEq, Eq)]
pub enum ResetTokenDestination {
    None,
    Email(ResetTokenDestinationEmail),
}

impl ResetTokenDestination {
    pub fn validate(
        destination: ResetTokenDestinationExtract,
    ) -> Result<ResetTokenDestination, ValidateResetTokenDestinationError> {
        match destination {
            ResetTokenDestinationExtract::None => Ok(ResetTokenDestination::None),
            ResetTokenDestinationExtract::Email(email) => {
                match ResetTokenDestinationEmail::validate(email) {
                    Ok(email) => Ok(ResetTokenDestination::Email(email)),
                    Err(err) => Err(ValidateResetTokenDestinationError::Email(err)),
                }
            }
        }
    }

    pub(in crate::auth) fn restore(
        destination: ResetTokenDestinationExtract,
    ) -> ResetTokenDestination {
        match destination {
            ResetTokenDestinationExtract::None => ResetTokenDestination::None,
            ResetTokenDestinationExtract::Email(email) => {
                ResetTokenDestination::Email(ResetTokenDestinationEmail::restore(email))
            }
        }
    }

    pub fn extract(self) -> ResetTokenDestinationExtract {
        match self {
            Self::None => ResetTokenDestinationExtract::None,
            Self::Email(email) => ResetTokenDestinationExtract::Email(email.extract()),
        }
    }
}

impl std::fmt::Display for ResetTokenDestination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "reset token destination: none"),
            Self::Email(email) => write!(f, "reset token destination: {}", email),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct ResetTokenDestinationEmail(String);

impl ResetTokenDestinationEmail {
    pub fn validate(
        email: impl ResetTokenDestinationEmailExtract,
    ) -> Result<Self, ValidateResetTokenDestinationEmailError> {
        Ok(Self(email.validate()?))
    }

    pub(in crate::auth) const fn restore(email: String) -> Self {
        Self(email)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

impl std::fmt::Display for ResetTokenDestinationEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "email({})", self.0)
    }
}

pub trait ResetTokenDestinationEmailExtract {
    fn validate(self) -> Result<String, ValidateResetTokenDestinationEmailError>;
}

pub enum ValidateResetTokenDestinationError {
    Email(ValidateResetTokenDestinationEmailError),
}

impl std::fmt::Display for ValidateResetTokenDestinationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Email(err) => err.fmt(f),
        }
    }
}

pub enum ValidateResetTokenDestinationEmailError {
    Invalid,
    Empty,
    TooLong,
}

impl std::fmt::Display for ValidateResetTokenDestinationEmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Invalid => write!(f, "invalid email"),
            Self::Empty => write!(f, "empty email"),
            Self::TooLong => write!(f, "too long email"),
        }
    }
}

#[derive(Clone)]
pub enum ResetTokenDestinationExtract {
    None,
    Email(String),
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
