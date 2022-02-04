pub enum RegisterResult<T> {
    Success(T),
    Conflict,
}

pub enum RepositoryError {
    InfraError(String),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "repository infra error: {}", err),
        }
    }
}
