use crate::auth::user::{
    login_id::kernel::data::ValidateLoginIdError, password::kernel::data::ValidatePasswordError,
};

pub enum ValidateAuthenticatePasswordFieldsError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidPassword(ValidatePasswordError),
}

impl std::fmt::Display for ValidateAuthenticatePasswordFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => write!(f, "login-id: {}", err),
            Self::InvalidPassword(err) => write!(f, "password: {}", err),
        }
    }
}
