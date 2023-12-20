use crate::auth::{
    proxy::data::AuthProxyCallError, ticket::kernel::data::ValidateAuthenticateTokenError,
};

#[derive(Debug)]
pub enum LogoutProxyError {
    Invalid(ValidateAuthenticateTokenError),
    ProxyError(AuthProxyCallError),
}

const ERROR: &'static str = "logout error";

impl std::fmt::Display for LogoutProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}; {}", ERROR, err),
            Self::ProxyError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<ValidateAuthenticateTokenError> for LogoutProxyError {
    fn from(value: ValidateAuthenticateTokenError) -> Self {
        Self::Invalid(value)
    }
}

impl From<AuthProxyCallError> for LogoutProxyError {
    fn from(value: AuthProxyCallError) -> Self {
        Self::ProxyError(value)
    }
}
