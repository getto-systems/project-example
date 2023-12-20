use crate::{
    auth::ticket::kernel::data::AuthTicket, common::api::repository::data::RepositoryError,
};

#[derive(Debug, PartialEq)]
pub struct LogoutSuccess(AuthTicket);

impl LogoutSuccess {
    pub(in crate::auth) fn new(ticket: AuthTicket) -> Self {
        Self(ticket)
    }
}

impl std::fmt::Display for LogoutSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub enum LogoutError {
    RepositoryError(RepositoryError),
}

const ERROR: &'static str = "logout error";

impl std::fmt::Display for LogoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RepositoryError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}

impl From<RepositoryError> for LogoutError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
