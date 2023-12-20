use crate::auth::{
    proxy::data::AuthProxyCallError,
    ticket::kernel::data::{
        AuthPermissionError, DecodeAuthorizeTokenError, ValidateAuthorizeTokenError,
    },
};

#[derive(Debug)]
pub enum AuthorizeProxyError {
    Invalid(ValidateAuthorizeTokenError),
    DecodeError(DecodeAuthorizeTokenError),
    PermissionError(AuthPermissionError),
    ProxyError(AuthProxyCallError),
}

const ERROR: &'static str = "authorize error";

impl std::fmt::Display for AuthorizeProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}; {}", ERROR, err),
            Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
            Self::PermissionError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ProxyError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<ValidateAuthorizeTokenError> for AuthorizeProxyError {
    fn from(value: ValidateAuthorizeTokenError) -> Self {
        Self::Invalid(value)
    }
}

impl From<DecodeAuthorizeTokenError> for AuthorizeProxyError {
    fn from(value: DecodeAuthorizeTokenError) -> Self {
        Self::DecodeError(value)
    }
}

impl From<AuthPermissionError> for AuthorizeProxyError {
    fn from(value: AuthPermissionError) -> Self {
        Self::PermissionError(value)
    }
}

impl From<AuthProxyCallError> for AuthorizeProxyError {
    fn from(value: AuthProxyCallError) -> Self {
        Self::ProxyError(value)
    }
}
