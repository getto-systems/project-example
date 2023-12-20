pub enum RegisterError {
    Conflict,
    Error(RepositoryError),
}

#[derive(Debug)]
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

impl<M: std::fmt::Display> From<(&'static str, M)> for RepositoryError {
    fn from((label, message): (&'static str, M)) -> Self {
        Self::InfraError(format!("{}; {}", label, message))
    }
}

#[derive(Debug)]
pub struct CurrentlyInUse<T> {
    pub id: T,
    pub used_by: String,
}

impl<T: std::fmt::Display> std::fmt::Display for CurrentlyInUse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "currently in use; {}; used by {}", self.id, self.used_by)
    }
}
