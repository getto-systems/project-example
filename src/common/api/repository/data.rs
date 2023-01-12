pub enum RegisterResultLEGACY<T> {
    Success(T),
    Conflict,
}

pub enum RegisterError {
    Conflict,
    Error(RepositoryError),
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
