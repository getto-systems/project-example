#[derive(Clone)]
pub struct ServiceAuthorizeToken(String);

impl ServiceAuthorizeToken {
    pub const fn restore(token: String) -> Self {
        Self(token)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

pub enum ServiceAuthorizeError {
    InfraError(String),
}

impl std::fmt::Display for ServiceAuthorizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InfraError(err) => write!(f, "service authorize error; {}", err),
        }
    }
}
