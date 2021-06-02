use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use super::super::kernel::data::ValidateAuthRolesError;
use crate::z_details::_api::{repository::data::RepositoryError, request::data::HeaderError};

#[derive(Debug)]
pub enum ValidateAuthTokenError {
    HeaderError(HeaderError),
    DecodeError(DecodeAuthTokenError),
    RepositoryError(RepositoryError),
    PermissionError(ValidateAuthRolesError),
}

impl Display for ValidateAuthTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let label = "auth token error";
        match self {
            Self::HeaderError(err) => write!(f, "{}: {}", label, err),
            Self::DecodeError(err) => write!(f, "{}: {}", label, err),
            Self::RepositoryError(err) => write!(f, "{}: {}", label, err),
            Self::PermissionError(err) => write!(f, "{}: {}", label, err),
        }
    }
}
impl Error for ValidateAuthTokenError {}

#[derive(Debug)]
pub enum DecodeAuthTokenError {
    Expired,
    Invalid(String),
}

impl Display for DecodeAuthTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Expired => write!(f, "token expired"),
            Self::Invalid(err) => write!(f, "invalid token: {}", err),
        }
    }
}
impl Error for DecodeAuthTokenError {}
