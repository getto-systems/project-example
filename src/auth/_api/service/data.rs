use std::fmt::Display;

pub enum AuthServiceError {
    InvalidArgument(String),
    AlreadyExists(String),
    Unauthenticated(String),
    PermissionDenied(String),
    Internal(String),
    Cancelled(String),
    InfraError(String),
}

impl Display for AuthServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArgument(err) => write!(f, "invalid argument; {}", err),
            Self::AlreadyExists(err) => write!(f, "already exists; {}", err),
            Self::Unauthenticated(err) => write!(f, "unauthenticated; {}", err),
            Self::PermissionDenied(err) => write!(f, "permission denied; {}", err),
            Self::Internal(err) => write!(f, "internal; {}", err),
            Self::Cancelled(err) => write!(f, "cancelled; {}", err),
            Self::InfraError(err) => write!(f, "infra error; {}", err),
        }
    }
}