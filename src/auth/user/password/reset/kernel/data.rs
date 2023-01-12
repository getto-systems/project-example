use crate::common::api::validate::data::ValidateTextError;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ResetPasswordId(String);

impl ResetPasswordId {
    pub const fn restore(value: String) -> Self {
        Self(value)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

pub struct ResetPasswordToken(String);

impl ResetPasswordToken {
    // TODO convert ではなく、AuthenticateToken と同じように扱いたい
    pub fn convert(
        value: impl ResetPasswordTokenExtract,
    ) -> Result<Self, ValidateResetPasswordTokenError> {
        Ok(Self(value.convert().map_err(
            ValidateResetPasswordTokenError::ResetPasswordToken,
        )?))
    }

    pub const fn restore(value: String) -> Self {
        Self(value)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

pub trait ResetPasswordTokenExtract {
    fn convert(self) -> Result<String, ValidateTextError>;
}

#[derive(Debug)]
pub enum ValidateResetPasswordTokenError {
    ResetPasswordToken(ValidateTextError),
}

impl std::fmt::Display for ValidateResetPasswordTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::ResetPasswordToken(err) => err.fmt(f),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ResetPasswordTokenDestination {
    None,
    Email(ResetPasswordTokenDestinationEmail),
}

impl std::fmt::Display for ResetPasswordTokenDestination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "reset token destination: none"),
            Self::Email(email) => write!(f, "reset token destination: {}", email),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct ResetPasswordTokenDestinationEmail(String);

impl ResetPasswordTokenDestinationEmail {
    pub fn convert(
        email: impl ResetPasswordTokenDestinationEmailExtract,
    ) -> Result<Self, ValidateResetPasswordTokenDestinationError> {
        Ok(Self(email.convert().map_err(
            ValidateResetPasswordTokenDestinationError::Email,
        )?))
    }

    pub(in crate::auth) const fn restore(email: String) -> Self {
        Self(email)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

impl std::fmt::Display for ResetPasswordTokenDestinationEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "email: {}", self.0)
    }
}

pub trait ResetPasswordTokenDestinationEmailExtract {
    fn convert(self) -> Result<String, ValidateTextError>;
}

#[derive(Debug)]
pub enum ValidateResetPasswordTokenDestinationError {
    NotFound,
    Email(ValidateTextError),
}

impl std::fmt::Display for ValidateResetPasswordTokenDestinationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotFound => write!(f, "data not found"),
            Self::Email(err) => write!(f, "reset-token-destination: email: {}", err),
        }
    }
}
