use crate::{
    auth::ticket::{
        authorize::data::CheckAuthorizeTokenError, kernel::data::ValidateAuthorizeTokenError,
    },
    common::api::{
        message::data::MessageError,
        service::data::{ServiceAuthorizeError, ServiceConnectError, ServiceMetadataError},
    },
};

#[derive(Clone)]
pub struct ProxyDomain(String);

impl ProxyDomain {
    pub fn restore(domain: String) -> Self {
        Self(domain)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub enum AuthProxyCallError {
    Unauthenticated(String),
    InfraError(String),
    CheckAuthorizeTokenError(CheckAuthorizeTokenError),
    ValidateAuthorizeTokenError(ValidateAuthorizeTokenError),
    ServiceAuthorizeError(ServiceAuthorizeError),
    ServiceConnectError(ServiceConnectError),
    ServiceMetadataError(ServiceMetadataError),
    MessageError(MessageError),
}

impl std::fmt::Display for AuthProxyCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unauthenticated(err) => write!(f, "unauthenticated; {}", err),
            Self::InfraError(err) => write!(f, "infra error; {}", err),
            Self::CheckAuthorizeTokenError(err) => {
                write!(f, "failed to check authorize-token; {}", err)
            }
            Self::ValidateAuthorizeTokenError(err) => write!(f, "invalid authorize-token; {}", err),
            Self::ServiceAuthorizeError(err) => write!(f, "service authorize error; {}", err),
            Self::ServiceConnectError(err) => write!(f, "service connect error; {}", err),
            Self::ServiceMetadataError(err) => write!(f, "service metadata error; {}", err),
            Self::MessageError(err) => write!(f, "message error; {}", err),
        }
    }
}

impl From<CheckAuthorizeTokenError> for AuthProxyCallError {
    fn from(value: CheckAuthorizeTokenError) -> Self {
        Self::CheckAuthorizeTokenError(value)
    }
}

impl From<ValidateAuthorizeTokenError> for AuthProxyCallError {
    fn from(value: ValidateAuthorizeTokenError) -> Self {
        Self::ValidateAuthorizeTokenError(value)
    }
}

impl From<ServiceAuthorizeError> for AuthProxyCallError {
    fn from(value: ServiceAuthorizeError) -> Self {
        Self::ServiceAuthorizeError(value)
    }
}

impl From<ServiceConnectError> for AuthProxyCallError {
    fn from(value: ServiceConnectError) -> Self {
        Self::ServiceConnectError(value)
    }
}

impl From<ServiceMetadataError> for AuthProxyCallError {
    fn from(value: ServiceMetadataError) -> Self {
        Self::ServiceMetadataError(value)
    }
}

impl From<MessageError> for AuthProxyCallError {
    fn from(value: MessageError) -> Self {
        Self::MessageError(value)
    }
}
