use std::fmt::Display;

use super::super::kernel::data::{AuthTicket, ValidateAuthNonceError};
use super::data::ValidateAuthTokenError;

pub enum ValidateAuthTokenEvent {
    Success(AuthTicket),
    NonceError(ValidateAuthNonceError),
    TokenError(ValidateAuthTokenError),
}

const SUCCESS: &'static str = "validate success";
const ERROR: &'static str = "validate error";

impl Display for ValidateAuthTokenEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(auth_ticket) => write!(f, "{}; {}", SUCCESS, auth_ticket),
            Self::NonceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::TokenError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}
