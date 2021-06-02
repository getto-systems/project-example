use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::auth::auth_ticket::_api::kernel::data::AuthTokenExtract;

#[derive(Clone)]
pub struct AuthTicketEncoded {
    pub message: String,
    pub ticket_tokens: Vec<AuthTokenEncoded>,
    pub api_tokens: Vec<AuthTokenEncoded>,
    pub cdn_tokens: Vec<AuthTokenEncoded>,
}

#[derive(Clone)]
pub struct AuthTokenEncoded {
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
