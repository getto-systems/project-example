use crate::auth::auth_ticket::_auth::kernel::data::ExpireDateTime;

#[derive(Clone)]
pub struct AuthTokenExpires {
    pub ticket: ExpireDateTime,
    pub api: ExpireDateTime,
    pub cloudfront: ExpireDateTime,
}

impl std::fmt::Display for AuthTokenExpires {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "ticket: {} / api: {} / cloudfront: {}",
            self.ticket, self.api, self.cloudfront
        )
    }
}

pub enum EncodeAuthTokenError {
    InfraError(String),
}

impl std::fmt::Display for EncodeAuthTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "encode error: {}", err),
        }
    }
}