use crate::auth::proxy::data::AuthProxyError;

pub fn infra_error(err: impl std::fmt::Display) -> AuthProxyError {
    AuthProxyError::InfraError(format!("proxy infra error; {}", err))
}
