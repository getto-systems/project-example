use crate::common::api::validate::data::ValidateTextError;

#[derive(Debug)]
pub enum ValidatePasswordError {
    Password(ValidateTextError),
}

impl std::fmt::Display for ValidatePasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Password(err) => write!(f, "password: {}", err),
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
