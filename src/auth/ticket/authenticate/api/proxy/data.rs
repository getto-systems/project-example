use crate::{
    auth::{
        proxy::data::{AuthProxyCallError, ProxyDomain},
        ticket::kernel::data::{AuthToken, ValidateAuthenticateTokenError},
    },
    common::proxy::data::ProxyResponseBody,
};

#[derive(Debug)]
pub enum AuthenticateWithTokenProxyError {
    Invalid(ValidateAuthenticateTokenError),
    ProxyError(AuthProxyCallError),
}

const ERROR: &'static str = "authenticate with token error";

impl std::fmt::Display for AuthenticateWithTokenProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}; {}", ERROR, err),
            Self::ProxyError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<ValidateAuthenticateTokenError> for AuthenticateWithTokenProxyError {
    fn from(value: ValidateAuthenticateTokenError) -> Self {
        Self::Invalid(value)
    }
}

impl From<AuthProxyCallError> for AuthenticateWithTokenProxyError {
    fn from(value: AuthProxyCallError) -> Self {
        Self::ProxyError(value)
    }
}

pub type ProxyResponseAuthenticated = (ProxyResponseBody, Option<(AuthToken, ProxyDomain)>);
