use crate::auth::ticket::kernel::data::{ValidateAuthPermissionError, ValidateAuthorizeTokenError};

pub enum ValidateAuthorizeFieldsError {
    Token(ValidateAuthorizeTokenError),
    Required(ValidateAuthPermissionError),
}

impl std::fmt::Display for ValidateAuthorizeFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Token(err) => write!(f, "invalid token: {}", err),
            Self::Required(err) => write!(f, "invalid permission: {}", err),
        }
    }
}
