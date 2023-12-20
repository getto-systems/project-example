use crate::auth::ticket::kernel::data::{
    AuthTicket, DecodeAuthenticateTokenError, ValidateAuthenticateTokenError,
};

#[derive(Debug, PartialEq)]
pub struct CheckAuthenticateTokenSuccess(AuthTicket);

impl CheckAuthenticateTokenSuccess {
    pub fn new(ticket: AuthTicket) -> Self {
        Self(ticket)
    }

    pub fn extract(self) -> AuthTicket {
        self.0
    }
}

impl std::fmt::Display for CheckAuthenticateTokenSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "check authenticate-token success; {}", self.0)
    }
}

#[derive(Debug)]
pub enum CheckAuthenticateTokenError {
    Invalid(ValidateAuthenticateTokenError),
    DecodeError(DecodeAuthenticateTokenError),
}

const ERROR: &'static str = "check authenticate-token error";

impl std::fmt::Display for CheckAuthenticateTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}; {}", ERROR, err),
            Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<ValidateAuthenticateTokenError> for CheckAuthenticateTokenError {
    fn from(value: ValidateAuthenticateTokenError) -> Self {
        Self::Invalid(value)
    }
}

impl From<DecodeAuthenticateTokenError> for CheckAuthenticateTokenError {
    fn from(value: DecodeAuthenticateTokenError) -> Self {
        Self::DecodeError(value)
    }
}
