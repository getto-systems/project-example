use crate::auth::kernel::data::ExpireDateTime;

#[derive(Clone)]
pub struct AuthTokenExpires {
    pub authenticate: ExpireDateTime,
    pub authorize: ExpireDateTime,
    pub cdn: ExpireDateTime,
}

impl std::fmt::Display for AuthTokenExpires {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "authenticate: {} / authorize: {} / cdn: {}",
            self.authenticate, self.authorize, self.cdn
        )
    }
}

pub enum EncodeAuthTokenError {
    InfraError(String),
}

impl std::fmt::Display for EncodeAuthTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "encode auth-token error: {}", err),
        }
    }
}
