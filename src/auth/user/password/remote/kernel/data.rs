use crate::z_lib::remote::repository::data::RepositoryError;

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

pub enum VerifyResetTokenEntryError {
    ResetTokenEntryNotFound,
    LoginIdNotMatched,
    Expired,
    AlreadyReset,
}

impl std::fmt::Display for VerifyResetTokenEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::ResetTokenEntryNotFound => write!(f, "reset token entry not found"),
            Self::LoginIdNotMatched => write!(f, "login id not matched"),
            Self::Expired => write!(f, "reset token expired"),
            Self::AlreadyReset => write!(f, "already reset"),
        }
    }
}

pub enum VerifyPasswordRepositoryError {
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
    PasswordNotFound,
    PasswordNotMatched,
}

pub enum ChangePasswordRepositoryError {
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
    PasswordNotFound,
    PasswordNotMatched,
}

pub enum RegisterResetTokenRepositoryError {
    RepositoryError(RepositoryError),
    UserNotFound,
}

pub enum ResetPasswordRepositoryError {
    RepositoryError(RepositoryError),
    PasswordHashError(PasswordHashError),
}

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
