use std::{
    error::Error,
    fmt::{Display, Formatter},
};

pub enum RegisterAttemptResult<T> {
    Success(T),
    Conflict,
}

#[derive(Debug)]
pub enum RepositoryError {
    InfraError(String),
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "infra error: {}", err),
        }
    }
}
impl Error for RepositoryError {}
