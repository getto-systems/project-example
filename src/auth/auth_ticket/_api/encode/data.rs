use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::auth::auth_ticket::_api::kernel::data::{AuthTokenExtract, ExpireDateTime};

#[derive(Clone)]
pub struct AuthTokenExpires {
    pub ticket: ExpireDateTime,
    pub api: ExpireDateTime,
    pub cdn: ExpireDateTime,
}

impl Display for AuthTokenExpires {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "ticket: {} / api: {} / cdn: {}", self.ticket, self.api, self.cdn)
    }
}

#[derive(Clone)]
pub struct AuthTokenEncoded {
    pub message: String,
    pub ticket_tokens: Vec<AuthTokenEncodedData>,
    pub api_tokens: Vec<AuthTokenEncodedData>,
    pub cdn_tokens: Vec<AuthTokenEncodedData>,
}

#[derive(Clone)]
pub struct AuthTokenEncodedData {
    pub domain: String,
    pub name: String,
    pub token: AuthTokenExtract,
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
