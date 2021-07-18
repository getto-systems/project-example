use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::auth::{
    auth_ticket::_auth::kernel::data::{AuthTokenExtract, ExpireDateTime},
    auth_user::_auth::kernel::data::GrantedAuthRoles,
};

#[derive(Clone)]
pub struct AuthTokenExpires {
    pub ticket: ExpireDateTime,
    pub api: ExpireDateTime,
    pub cloudfront: ExpireDateTime,
}

impl Display for AuthTokenExpires {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "ticket: {} / api: {} / cloudfront: {}",
            self.ticket, self.api, self.cloudfront
        )
    }
}

pub struct AuthTokenEncoded {
    pub granted_roles: GrantedAuthRoles,
    pub ticket_tokens: Vec<AuthTokenEncodedData>,
    pub api_tokens: Vec<AuthTokenEncodedData>,
    pub cloudfront_tokens: Vec<AuthTokenEncodedData>,
}

pub struct AuthTokenEncodedData {
    pub kind: AuthTokenKind,
    pub token: AuthTokenExtract,
}

pub enum AuthTokenKind {
    Ticket,
    Api,
    CloudfrontKeyPairId,
    CloudfrontPolicy,
    CloudfrontSignature,
}

#[derive(Debug)]
pub enum EncodeAuthTokenError {
    InfraError(String),
}

impl Display for EncodeAuthTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "encode error: {}", err),
        }
    }
}
impl Error for EncodeAuthTokenError {}
