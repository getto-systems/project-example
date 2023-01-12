use tonic::{metadata::errors::InvalidMetadataValue, transport::Error as TransportError};
use url::ParseError;

use crate::common::api::request::data::MetadataError;

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

pub enum ServiceConnectError {
    UrlParseError(ParseError),
    InvalidUrlError,
    TransportError(TransportError),
}

impl std::fmt::Display for ServiceConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UrlParseError(err) => write!(f, "url parse error; {}", err),
            Self::InvalidUrlError => write!(f, "invalid url"),
            Self::TransportError(err) => write!(f, "transport error; {}", err),
        }
    }
}

pub enum ServiceMetadataError {
    MetadataError(MetadataError),
    InvalidMetadataValue(InvalidMetadataValue),
    AuthorizeError(ServiceAuthorizeError),
}

impl std::fmt::Display for ServiceMetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MetadataError(err) => write!(f, "metadata error; {}", err),
            Self::InvalidMetadataValue(err) => write!(f, "invalid metadata value; {}", err),
            Self::AuthorizeError(err) => write!(f, "service authorize error; {}", err),
        }
    }
}
