use crate::{
    auth::{
        kernel::data::ExpireDateTime,
        ticket::kernel::data::{AuthPermissionGranted, AuthToken},
    },
    common::api::repository::data::RepositoryError,
};

#[derive(Clone)]
pub struct AuthTokenExpires {
    pub authenticate: ExpireDateTime,
    pub authorize: ExpireDateTime,
    pub cdn: ExpireDateTime,
}

impl std::fmt::Display for AuthTokenExpires {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "authenticate: {} / authorize: {} / cdn: {}",
            self.authenticate, self.authorize, self.cdn
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct EncodeAuthTokenSuccess(AuthToken, AuthPermissionGranted);

impl EncodeAuthTokenSuccess {
    pub fn new(token: AuthToken, granted: AuthPermissionGranted) -> Self {
        Self(token, granted)
    }

    pub fn extract(self) -> (AuthToken, AuthPermissionGranted) {
        (self.0, self.1)
    }
}

impl std::fmt::Display for EncodeAuthTokenSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "encode auth-token success; {}", self.1)
    }
}

#[derive(Debug)]
pub enum EncodeAuthTokenError {
    TicketNotFound,
    RepositoryError(RepositoryError),
    EncodeError(EncodeTokenError),
}

const ERROR: &'static str = "encode auth-token error";

impl std::fmt::Display for EncodeAuthTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TicketNotFound => write!(f, "{}; ticket not found", ERROR),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::EncodeError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<RepositoryError> for EncodeAuthTokenError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

impl From<EncodeTokenError> for EncodeAuthTokenError {
    fn from(value: EncodeTokenError) -> Self {
        Self::EncodeError(value)
    }
}

#[derive(Debug)]
pub enum EncodeTokenError {
    InfraError(String),
}

impl std::fmt::Display for EncodeTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "encode auth-token error: {}", err),
        }
    }
}
