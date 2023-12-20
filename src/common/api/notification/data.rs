#[derive(Debug)]
pub enum NotificationError {
    InfraError(String),
}

impl std::fmt::Display for NotificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "send email infra error: {}", err),
        }
    }
}

impl<M: std::fmt::Display> From<(&'static str, M)> for NotificationError {
    fn from((label, message): (&'static str, M)) -> Self {
        Self::InfraError(format!("{}; {}", label, message))
    }
}
