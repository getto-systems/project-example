use crate::common::api::{
    message::data::MessageError,
    service::data::{ServiceConnectError, ServiceMetadataError},
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

pub enum AuthProxyError {
    Unauthenticated(String),
    InfraError(String),
    ServiceConnectError(ServiceConnectError),
    ServiceMetadataError(ServiceMetadataError),
    MessageError(MessageError),
}

impl std::fmt::Display for AuthProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unauthenticated(err) => write!(f, "unauthenticated; {}", err),
            Self::InfraError(err) => write!(f, "infra error; {}", err),
            Self::ServiceConnectError(err) => write!(f, "service connect error; {}", err),
            Self::ServiceMetadataError(err) => write!(f, "service metadata error; {}", err),
            Self::MessageError(err) => write!(f, "message error; {}", err),
        }
    }
}
