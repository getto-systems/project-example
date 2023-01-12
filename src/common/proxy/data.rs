use std::collections::HashMap;

use crate::common::api::{
    message::data::MessageError,
    request::data::MetadataError,
    service::data::{ServiceConnectError, ServiceMetadataError},
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
    PermissionDenied(String),
    InfraError(String),
    ServiceConnectError(ServiceConnectError),
    ServiceMetadataError(ServiceMetadataError),
    MessageError(MessageError),
}

impl std::fmt::Display for CoreProxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PermissionDenied(err) => write!(f, "permission denied; {}", err),
            Self::InfraError(err) => write!(f, "infra error; {}", err),
            Self::ServiceConnectError(err) => write!(f, "service connect error; {}", err),
            Self::ServiceMetadataError(err) => write!(f, "service metadata error; {}", err),
            Self::MessageError(err) => write!(f, "message error; {}", err),
        }
    }
}
