use std::collections::HashMap;

use crate::{
    auth::data::{AuthorizeWithTokenError, CheckAuthorizeTokenError, ValidateAuthorizeTokenError},
    common::api::{
        message::data::MessageError,
        request::data::MetadataError,
        service::data::{ServiceAuthorizeError, ServiceConnectError, ServiceMetadataError},
    },
};

pub trait ProxyMetadataExtract: Send {
    fn convert(self) -> Result<HashMap<&'static str, String>, MetadataError>;
}

impl ProxyMetadataExtract for () {
    fn convert(self) -> Result<HashMap<&'static str, String>, MetadataError> {
        Ok(Default::default())
    }
}

pub struct ProxyResponseBody(String);

impl ProxyResponseBody {
    pub fn restore(body: String) -> Self {
        Self(body)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

pub enum CoreProxyError {
    AuthorizeWithTokenError(AuthorizeWithTokenError),
    CoreProxyCallError(CoreProxyCallError),
}

impl std::fmt::Display for CoreProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthorizeWithTokenError(err) => err.fmt(f),
            Self::CoreProxyCallError(err) => err.fmt(f),
        }
    }
}

impl From<AuthorizeWithTokenError> for CoreProxyError {
    fn from(value: AuthorizeWithTokenError) -> Self {
        Self::AuthorizeWithTokenError(value)
    }
}

impl From<CoreProxyCallError> for CoreProxyError {
    fn from(value: CoreProxyCallError) -> Self {
        Self::CoreProxyCallError(value)
    }
}

pub enum CoreProxyCallError {
    PermissionDenied(String),
    InfraError(String),
    CheckAuthorizeTokenError(CheckAuthorizeTokenError),
    ValidateAuthorizeTokenError(ValidateAuthorizeTokenError),
    ServiceConnectError(ServiceConnectError),
    ServiceMetadataError(ServiceMetadataError),
    ServiceAuthorizeError(ServiceAuthorizeError),
    MessageError(MessageError),
}

impl std::fmt::Display for CoreProxyCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PermissionDenied(err) => write!(f, "permission denied; {}", err),
            Self::InfraError(err) => write!(f, "infra error; {}", err),
            Self::CheckAuthorizeTokenError(err) => {
                write!(f, "failed to check authorize-token; {}", err)
            }
            Self::ValidateAuthorizeTokenError(err) => write!(f, "invalid authorize-token; {}", err),
            Self::ServiceConnectError(err) => write!(f, "service connect error; {}", err),
            Self::ServiceMetadataError(err) => write!(f, "service metadata error; {}", err),
            Self::ServiceAuthorizeError(err) => write!(f, "service authorize error; {}", err),
            Self::MessageError(err) => write!(f, "message error; {}", err),
        }
    }
}

impl From<CheckAuthorizeTokenError> for CoreProxyCallError {
    fn from(value: CheckAuthorizeTokenError) -> Self {
        Self::CheckAuthorizeTokenError(value)
    }
}

impl From<ValidateAuthorizeTokenError> for CoreProxyCallError {
    fn from(value: ValidateAuthorizeTokenError) -> Self {
        Self::ValidateAuthorizeTokenError(value)
    }
}

impl From<ServiceConnectError> for CoreProxyCallError {
    fn from(value: ServiceConnectError) -> Self {
        Self::ServiceConnectError(value)
    }
}

impl From<ServiceMetadataError> for CoreProxyCallError {
    fn from(value: ServiceMetadataError) -> Self {
        Self::ServiceMetadataError(value)
    }
}

impl From<ServiceAuthorizeError> for CoreProxyCallError {
    fn from(value: ServiceAuthorizeError) -> Self {
        Self::ServiceAuthorizeError(value)
    }
}

impl From<MessageError> for CoreProxyCallError {
    fn from(value: MessageError) -> Self {
        Self::MessageError(value)
    }
}
