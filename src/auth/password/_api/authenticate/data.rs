use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::z_details::_api::repository::data::RepositoryError;

pub struct AuthenticatePasswordResponse {
    pub message: String,
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
pub enum PasswordMatchError {
    InfraError(String),
}

impl Display for PasswordMatchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "password hash error: {}", err),
        }
    }
}
impl Error for PasswordMatchError {}

impl Into<RepositoryError> for PasswordMatchError {
    fn into(self) -> RepositoryError {
        match self {
            Self::InfraError(err) => RepositoryError::InfraError(err),
        }
    }
}
