mod x_actix_web;
mod x_tonic;

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct RequestInfo {
    id: String,
    path: String,
    method: String,
}

impl RequestInfo {
    pub fn extract(self) -> RequestInfoExtract {
        RequestInfoExtract {
            id: self.id,
            path: self.path,
            method: self.method,
        }
    }
}

impl Default for RequestInfo {
    fn default() -> Self {
        Self {
            id: "(no request-id)".to_owned(),
            path: "(no path)".to_owned(),
            method: "(no method)".to_owned(),
        }
    }
}

pub struct RequestInfoExtract {
    pub id: String,
    pub path: String,
    pub method: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestId(String);

impl RequestId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn not_specified() -> Self {
        Self("(no request-id)".to_owned())
    }

    pub fn restore(value: String) -> Self {
        Self(value)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub enum MetadataError {
    Invalid(String),
}

impl std::fmt::Display for MetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Invalid(err) => write!(f, "invalid metadata: {}", err),
        }
    }
}
