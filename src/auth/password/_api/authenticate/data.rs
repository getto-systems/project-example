use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::z_details::_api::repository::data::RepositoryError;

pub struct AuthenticatePasswordError {
    pub message: String,
}

#[derive(Debug)]
pub enum ConvertPasswordError {
    Empty,
    TooLong,
}

impl Display for ConvertPasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Empty => write!(f, "empty password"),
            Self::TooLong => write!(f, "too long password"),
        }
    }
}
impl Error for ConvertPasswordError {}

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

impl Into<RepositoryError> for PasswordHashError {
    fn into(self) -> RepositoryError {
        match self {
            Self::InfraError(err) => RepositoryError::InfraError(err),
        }
    }
}
