use crate::auth::auth_ticket::{
    _auth::kernel::data::{AuthTicket, ValidateAuthNonceError, ValidateAuthRolesError},
    remote::validate::data::ValidateAuthTokenError,
};

pub enum ValidateAuthTokenEvent {
    Success(AuthTicket),
    NonceError(ValidateAuthNonceError),
    TokenError(ValidateAuthTokenError),
    PermissionError(ValidateAuthRolesError),
}

const SUCCESS: &'static str = "validate success";
const ERROR: &'static str = "validate error";

impl std::fmt::Display for ValidateAuthTokenEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(ticket) => write!(f, "{}; {}", SUCCESS, ticket),
            Self::NonceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::TokenError(err) => write!(f, "{}; {}", ERROR, err),
            Self::PermissionError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}
