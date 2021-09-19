use std::collections::{HashMap, HashSet};

use crate::z_details::_common::request::data::MetadataError;

#[derive(Clone)]
pub struct AuthNonce(String);

impl AuthNonce {
    pub const fn restore(nonce: String) -> Self {
        Self(nonce)
    }

    pub fn extract(self) -> String {
        self.0
    }

    #[cfg(test)]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone)]
pub struct AuthToken(String);

impl AuthToken {
    pub const fn restore(token: String) -> Self {
        Self(token)
    }

    pub fn extract(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone)]
pub struct AuthTokenExtract {
    pub token: String,
    pub expires: i64,
}

pub struct AuthTokenEncoded {
    pub ticket_token: AuthTokenExtract,
    pub api_token: AuthTokenExtract,
    pub cloudfront_tokens: HashMap<CloudfrontTokenKind, AuthTokenExtract>,
}

#[derive(Eq, PartialEq, Hash)]
pub enum CloudfrontTokenKind {
    KeyPairId,
    Policy,
    Signature,
}

#[derive(Clone)]
pub struct AuthTicketExtract {
    pub ticket_id: String,
    pub user_id: String,
    pub granted_roles: HashSet<String>,
}

pub enum AuthServiceMetadataError {
    MetadataError(MetadataError),
    DecodeError(DecodeAuthTokenError),
}

impl std::fmt::Display for AuthServiceMetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::MetadataError(err) => err.fmt(f),
            Self::DecodeError(err) => err.fmt(f),
        }
    }
}

pub enum DecodeAuthTokenError {
    Expired,
    Invalid(String),
}

impl std::fmt::Display for DecodeAuthTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Expired => write!(f, "token expired"),
            Self::Invalid(err) => write!(f, "invalid token: {}", err),
        }
    }
}
