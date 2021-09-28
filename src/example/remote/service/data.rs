pub enum ExampleServiceError {
    Internal(String),
    Cancelled(String),
    InfraError(String),
}

impl std::fmt::Display for ExampleServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Internal(err) => write!(f, "internal; {}", err),
            Self::Cancelled(err) => write!(f, "cancelled; {}", err),
            Self::InfraError(err) => write!(f, "infra error; {}", err),
        }
    }
}
