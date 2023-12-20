use crate::common::api::repository::data::RepositoryError;

#[derive(Debug)]
pub enum IssueAuthTicketError {
    RepositoryError(RepositoryError),
}

const ERROR: &'static str = "issue auth-ticket error";

impl std::fmt::Display for IssueAuthTicketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RepositoryError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}

impl From<RepositoryError> for IssueAuthTicketError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
