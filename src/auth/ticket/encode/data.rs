use crate::auth::{
    ticket::kernel::data::{EncodedAuthTokens, ExpireDateTime},
    user::kernel::data::GrantedAuthRoles,
};

pub struct AuthTicketEncoded {
    pub roles: GrantedAuthRoles,
    pub token: EncodedAuthTokens,
}

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
