use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::auth::auth_ticket::_auth::kernel::data::ExpireDateTime;

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
